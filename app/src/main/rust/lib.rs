#![cfg(target_os = "android")]

pub use error::{Error, ErrorExt, ResultExt};

#[macro_use]
mod logger;

#[allow(dead_code)]
mod bridge;
mod constants;
mod counter;
mod error;
mod http_client;
mod message;
mod prelude;
mod types;

pub type Result<T> = std::result::Result<T, Error>;
