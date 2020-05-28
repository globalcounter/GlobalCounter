use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use tokio_tungstenite::tungstenite::Message;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WsMessage {
    pub counter: i64,
}

impl TryFrom<Message> for WsMessage {
    type Error = crate::Error;

    fn try_from(msg: Message) -> Result<Self, Self::Error> {
        let bytes = msg.into_data();
        serde_json::from_slice::<WsMessage>(&bytes).context("Failed to convert a `Message` to `WsMessage`")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CounterValueResponse {
    pub status: CounterValueResponseStatus,
    pub code: u16,
    #[serde(default)]
    pub data: Option<CounterValue>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CounterValueResponseStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CounterValue {
    pub counter: i64,
}
