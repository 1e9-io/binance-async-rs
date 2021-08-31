use crate::{
    client::Binance,
    model::{ExchangeInfo, ExchangeInformation, ServerTime},
};
use anyhow::Result;

impl Binance {
    // Test connectivity
    pub async fn ping(&self) -> Result<String> {
        Ok(self.transport.get::<_, ()>("/api/v1/ping", None).await?)
    }

    // Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        Ok(self.transport.get::<_, ()>("/api/v1/time", None).await?)
    }

    pub async fn get_exchange_info(&self) -> Result<ExchangeInfo> {
        Ok(self
            .transport
            .get::<_, ()>("/api/v1/exchangeInfo", None)
            .await?)
    }

    // Obtain exchange information (rate limits, symbol metadata etc)
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        Ok(self
            .transport
            .get::<_, ()>("/api/v1/exchangeInfo", None)
            .await?)
    }
}
