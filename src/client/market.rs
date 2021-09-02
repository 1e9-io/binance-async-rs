use super::Binance;
use crate::error::Error;
use crate::model::{BookTickers, SymbolPrice, Ticker};
use crate::model::{HistoricalTrade, KlineSummaries, KlineSummary, OrderBook, PriceStats, Prices};
use crate::transport::Version;
use anyhow::Result;
use log::debug;
use serde_json::{json, Value};
use std::{collections::HashMap, iter::FromIterator};

// Market Data endpoints
impl Binance {
    // Order book (Default 100; max 100)
    pub async fn get_depth<L>(&self, symbol: &str, limit: L) -> Result<OrderBook>
    where
        L: Into<Option<u64>>,
    {
        let limit = limit.into().unwrap_or(100);
        let params = json! {{"symbol": symbol.to_uppercase(), "limit": limit}};

        Ok(self
            .transport
            .get(Version::V3, "/depth", Some(params))
            .await?)
    }

    // Latest price for ALL symbols.
    pub async fn get_all_prices(&self) -> Result<Prices> {
        Ok(self
            .transport
            .get::<_, ()>(Version::V3, "/ticker/price", None)
            .await?)
    }

    pub async fn get_price(&self, symbol: &str) -> Result<SymbolPrice> {
        let symbol = symbol.to_uppercase();
        let params = json! {{"symbol": symbol.to_uppercase()}};
        Ok(self
            .transport
            .get(Version::V3, "/ticker/price", Some(params))
            .await?)
    }

    pub async fn get_historical_trades<L, F>(
        &self,
        symbol: &str,
        limit: L,
        from_id: F,
    ) -> Result<Vec<HistoricalTrade>>
    where
        L: Into<Option<u16>>,
        F: Into<Option<u64>>,
    {
        let limit = limit.into().unwrap_or(500);
        let params = match from_id.into() {
            Some(v) => json! {{"symbol": symbol.to_uppercase(), "limit": limit, "fromId": v}},
            None => json! {{"symbol": symbol.to_uppercase(), "limit": limit}},
        };
        debug!("params: {}", params.to_string());

        Ok(self
            .transport
            .get(Version::V3, "/historicalTrades", Some(params))
            .await?)
    }

    // Symbols order book ticker
    // -> Best price/qty on the order book for ALL symbols.
    pub async fn get_all_book_tickers(&self) -> Result<BookTickers> {
        Ok(self
            .transport
            .get::<_, ()>(Version::V1, "/ticker/allBookTickers", None)
            .await?)
    }

    // -> Best price/qty on the order book for ONE symbol
    pub async fn get_book_ticker(&self, symbol: &str) -> Result<Ticker> {
        let symbol = symbol.to_uppercase();
        let all_book_tickers = self.get_all_book_tickers();

        let BookTickers::AllBookTickers(book_tickers) = all_book_tickers.await?;

        Ok(book_tickers
            .into_iter()
            .find(|obj| obj.symbol == symbol)
            .ok_or_else(|| Error::SymbolNotFound)?)
    }

    // 24hr ticker price change statistics
    pub async fn get_24h_price_stats(&self, symbol: &str) -> Result<PriceStats> {
        let params = json! {{"symbol": symbol}};
        Ok(self
            .transport
            .get(Version::V1, "/ticker/24hr", Some(params))
            .await?)
    }

    // Returns up to 'limit' klines for given symbol and interval ("1m", "5m", ...)
    // https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
    pub async fn get_klines<S3, S4, S5>(
        &self,
        symbol: &str,
        interval: &str,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<KlineSummaries>
    where
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let mut params = vec![
            ("symbol", symbol.to_uppercase()),
            ("interval", interval.to_string()),
        ];

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            params.push(("limit", lt.to_string()));
        }
        if let Some(st) = start_time.into() {
            params.push(("startTime", st.to_string()));
        }
        if let Some(et) = end_time.into() {
            params.push(("endTime", et.to_string()));
        }
        let params: HashMap<&str, String> = HashMap::from_iter(params);

        let f = self.transport.get(Version::V3, "/klines", Some(params));

        let data: Vec<Vec<Value>> = f.await?;

        Ok(KlineSummaries::AllKlineSummaries(
            data.iter()
                .map(|row| KlineSummary {
                    open_time: to_i64(&row[0]),
                    open: to_f64(&row[1]),
                    high: to_f64(&row[2]),
                    low: to_f64(&row[3]),
                    close: to_f64(&row[4]),
                    volume: to_f64(&row[5]),
                    close_time: to_i64(&row[6]),
                    quote_asset_volume: to_f64(&row[7]),
                    number_of_trades: to_i64(&row[8]),
                    taker_buy_base_asset_volume: to_f64(&row[9]),
                    taker_buy_quote_asset_volume: to_f64(&row[10]),
                })
                .collect(),
        ))
    }

    // 24hr ticker price change statistics
    pub async fn get_24h_price_stats_all(&self) -> Result<Vec<PriceStats>> {
        Ok(self
            .transport
            .get::<_, ()>(Version::V1, "/ticker/24hr", None)
            .await?)
    }
}

fn to_i64(v: &Value) -> i64 {
    v.as_i64().unwrap()
}

fn to_f64(v: &Value) -> f64 {
    v.as_str().unwrap().parse().unwrap()
}

#[cfg(test)]
mod test {
    use crate::tests::test::setup;
    use anyhow::Result;

    #[tokio::test]
    async fn test_get_depth() -> Result<()> {
        let b = setup()?;
        b.get_depth("btcusdt", None).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_depth_with_limit() -> Result<()> {
        let b = setup()?;
        b.get_depth("btcusdt", 50).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_prices() -> Result<()> {
        let b = setup()?;
        b.get_all_prices().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_price() -> Result<()> {
        let b = setup()?;
        b.get_price("btcusdt").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_historical_trades() -> Result<()> {
        let b = setup()?;
        b.get_historical_trades("btcusdt", None, None).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_klines() -> Result<()> {
        let b = setup()?;
        b.get_klines("btcusdt", "5m", None, None, None).await?;
        Ok(())
    }
}
