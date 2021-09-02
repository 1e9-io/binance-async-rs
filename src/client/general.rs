use crate::transport::Version;
use crate::{
    client::Binance,
    model::{ExchangeInfo, ExchangeInformation, ServerTime},
};
use anyhow::Result;

impl Binance {
    // Test connectivity
    pub async fn ping(&self) -> Result<String> {
        Ok(self
            .transport
            .get::<_, ()>(Version::V3, "/ping", None)
            .await?)
    }

    // Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        Ok(self
            .transport
            .get::<_, ()>(Version::V3, "/time", None)
            .await?)
    }

    pub async fn get_exchange_info(&self) -> Result<ExchangeInfo> {
        Ok(self
            .transport
            .get::<_, ()>(Version::V3, "/exchangeInfo", None)
            .await?)
    }

    // Obtain exchange information (rate limits, symbol metadata etc)
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        Ok(self
            .transport
            .get::<_, ()>(Version::V3, "/exchangeInfo", None)
            .await?)
    }
}

#[cfg(test)]
mod test {
    use crate::Binance;
    use anyhow::Result;

    #[tokio::test]
    #[ignore] // TODO broken endpoint
    async fn test_ping() -> Result<()> {
        let b = Binance::new();
        b.ping().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_server_time() -> Result<()> {
        let b = Binance::new();
        b.get_server_time().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_exchange_info() -> Result<()> {
        let b = Binance::new();
        b.get_exchange_info().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_exchange_info() -> Result<()> {
        let b = Binance::new();
        b.exchange_info().await?;
        Ok(())
    }
}
