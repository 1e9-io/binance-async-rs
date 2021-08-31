use serde::{Deserialize, Serialize};
use thiserror::Error;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Error,Deserialize, Serialize, Debug, Clone)]
pub enum Error {
    #[error("Binance error: {}: {}", code, msg)]
    BinanceError { code: i64, msg: String },
    #[error("Assets not found")]
    AssetsNotFound,
    #[error("Symbol not found")]
    SymbolNotFound,
    #[error("No Api key set for private api")]
    NoApiKeySet,
    #[error("No stream is subscribed")]
    NoStreamSubscribed,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BinanceErrorData {
    pub code: i64,
    pub msg: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BinanceResponse<T> {
    Success(T),
    Error(BinanceErrorData),
}

impl<T: for<'a> Deserialize<'a>> BinanceResponse<T> {
    pub fn into_result(self) -> Result<T, Error> {
        match self {
            Self::Success(t) => Result::Ok(t),
            Self::Error(BinanceErrorData { code, msg }) => {
                Result::Err(Error::BinanceError { code, msg })
            }
        }
    }
}
