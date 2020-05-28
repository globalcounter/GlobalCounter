use crate::types::WsMessage;
use crate::{
    bridge::{self, MessageData},
    counter,
    message::what,
};
use crate::{
    constants,
    counter::{publish_counter_value, send_error},
};
use crate::{ErrorExt, ResultExt};
use futures::StreamExt;
use lazy_static::lazy_static;
use std::convert::TryFrom;
use std::sync::Mutex;
use tokio::time::{self, Duration};

lazy_static! {
    static ref IS_WS_INITIALIZED: Mutex<bool> = Mutex::new(false);
}

pub async fn init_ws() {
    {
        let mut lock = IS_WS_INITIALIZED.lock().unwrap();
        if *lock {
            return;
        }
        *lock = true;
    }

    start_daemon_ws_conn().await;
}

pub async fn start_daemon_ws_conn() {
    while let Err(err) = try_ws_conn().await {
        send_error(err.into_message());
        error!("WebSocket connection is closed, trying again");
        time::delay_for(Duration::from_secs(10)).await;
    }
}

pub async fn try_ws_conn() -> crate::Result<()> {
    let (mut ws, _) = tokio_tungstenite::connect_async(constants::API_SERVER_ENDPOINT_WS_CONNECT)
        .await
        .context("Failed to create WebSocket connection from the API server")?;

    bridge::send_message(
        what::SNACK_BAR_MSG,
        MessageData::new().put_string("msg", "Connected with API server"),
    );

    counter::fetch_and_publish_counter_value().await;

    while let Some(msg) = ws.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(err) => {
                let _ = ws.close(None).await;
                return Err(err.context("Failed to get next WS message"));
            }
        };

        if !msg.is_text() {
            continue;
        }

        let msg = match WsMessage::try_from(msg) {
            Ok(msg) => msg,
            Err(err) => {
                let _ = ws.close(None).await;
                return Err(err.context("Failed to parse WS message"));
            }
        };

        publish_counter_value(msg.counter);
    }

    Ok(())
}
