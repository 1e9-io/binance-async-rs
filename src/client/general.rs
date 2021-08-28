use crate::{
    client::Binance,
    model::{ExchangeInfo, ExchangeInformation, ServerTime},
};
use failure::Fallible;

impl Binance {
    // Test connectivity
    pub async fn ping(&self) -> Fallible<String> {
        Ok(self.transport.get::<_, ()>("/api/v1/ping", None).await?)
    }

    // Check server time
    pub async fn get_server_time(&self) -> Fallible<ServerTime> {
        Ok(self.transport.get::<_, ()>("/api/v1/time", None).await?)
    }

    pub async fn get_exchange_info(&self) -> Fallible<ExchangeInfo> {
        Ok(self
            .transport
            .get::<_, ()>("/api/v1/exchangeInfo", None)
            .await?)
    }

    // Obtain exchange information (rate limits, symbol metadata etc)
    pub async fn exchange_info(&self) -> Fallible<ExchangeInformation> {
        Ok(self
            .transport
            .get::<_, ()>("/api/v1/exchangeInfo", None)
            .await?)
    }
}
