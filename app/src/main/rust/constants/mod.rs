pub const APP_NAME: &'static str = "GlobalCounter";
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[cfg(debug_assertions)]
pub const API_SERVER_ENDPOINT_INCREASE_COUNTER: &'static str = "https://de53d94650bf.ngrok.io/api/v1/counter/increase";
#[cfg(debug_assertions)]
pub const API_SERVER_ENDPOINT_DECREASE_COUNTER: &'static str = "https://de53d94650bf.ngrok.io/api/v1/counter/decrease";
#[cfg(debug_assertions)]
pub const API_SERVER_ENDPOINT_GET_COUNTER_VALUE: &'static str = "https://de53d94650bf.ngrok.io/api/v1/counter/value";
#[cfg(debug_assertions)]
pub const API_SERVER_ENDPOINT_WS_CONNECT: &'static str = "wss://de53d94650bf.ngrok.io/ws/v1/connect";

#[cfg(not(debug_assertions))]
pub const API_SERVER_ENDPOINT_INCREASE_COUNTER: &'static str =
    "https://global-counter.herokuapp.com/api/v1/counter/increase";
#[cfg(not(debug_assertions))]
pub const API_SERVER_ENDPOINT_DECREASE_COUNTER: &'static str =
    "https://global-counter.herokuapp.com/api/v1/counter/decrease";
#[cfg(not(debug_assertions))]
pub const API_SERVER_ENDPOINT_GET_COUNTER_VALUE: &'static str =
    "https://global-counter.herokuapp.com/api/v1/counter/value";
#[cfg(not(debug_assertions))]
pub const API_SERVER_ENDPOINT_WS_CONNECT: &'static str = "wss://global-counter.herokuapp.com/ws/v1/connect";
