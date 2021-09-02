#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]

mod client;
pub mod error;
pub mod model;
mod transport;
mod tests;

pub use crate::client::{websocket::BinanceWebsocket, Binance};
