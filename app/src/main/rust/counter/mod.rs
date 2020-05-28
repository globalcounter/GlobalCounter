use crate::bridge::{self, MessageData};
use crate::message::what;
use crate::ResultExt;
use crate::{
    constants, http_client,
    types::{CounterValueResponse, CounterValueResponseStatus},
};

mod ws;

pub async fn on_initiate() {
    tokio::spawn(async move {
        fetch_and_publish_counter_value().await;
    });

    tokio::spawn(async move {
        ws::init_ws().await;
    });
}

pub async fn fetch_and_publish_counter_value() {
    let resp = http_client::client()
        .get(constants::API_SERVER_ENDPOINT_GET_COUNTER_VALUE)
        .send()
        .await;

    if let Err(err) = process_counter_resp(resp).await {
        send_error(err.into_message());
    }
}

pub async fn on_increase_counter() {
    let resp = http_client::client()
        .post(constants::API_SERVER_ENDPOINT_INCREASE_COUNTER)
        .send()
        .await;

    if let Err(err) = process_counter_resp(resp).await {
        send_error(err.into_message());
    }
}

pub async fn on_decrease_counter() {
    let resp = http_client::client()
        .post(constants::API_SERVER_ENDPOINT_DECREASE_COUNTER)
        .send()
        .await;

    if let Err(err) = process_counter_resp(resp).await {
        send_error(err.into_message());
    }
}

async fn process_counter_resp(resp: Result<reqwest::Response, reqwest::Error>) -> crate::Result<()> {
    let resp = resp.context("Failed to connect with the API server")?;

    let resp_data = resp
        .json::<CounterValueResponse>()
        .await
        .context("Failed to parse API server response")?;

    let counter_value = match resp_data.status {
        CounterValueResponseStatus::Success => match resp_data.data {
            Some(data) => data.counter,
            None => {
                return Err(crate::Error::new("No counter value is sent from the API server"));
            }
        },
        CounterValueResponseStatus::Failed => {
            return Err(crate::Error::new(format!(
                "API server error: {}",
                resp_data.message.unwrap_or("".to_owned())
            )));
        }
    };

    publish_counter_value(counter_value);

    Ok(())
}

pub fn publish_counter_value(value: i64) {
    let data = MessageData::new().put_string("value", value.to_string());
    bridge::send_message(what::COUNTER_VALUE, data);
}

pub fn send_error<M: AsRef<str>>(msg: M) {
    error!("{}", msg.as_ref());
    let data = MessageData::new().put_string("msg", msg);
    bridge::send_message(what::ERROR, data);
}
