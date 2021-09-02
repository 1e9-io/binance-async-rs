use crate::error::Error;
use crate::transport::Version;
use crate::{
    client::Binance,
    model::{AccountInformation, Balance, Order, OrderCanceled, TradeHistory, Transaction},
};
use anyhow::Result;
use serde_json::json;
use std::collections::HashMap;

const ORDER_TYPE_LIMIT: &str = "LIMIT";
const ORDER_TYPE_MARKET: &str = "MARKET";
const ORDER_SIDE_BUY: &str = "BUY";
const ORDER_SIDE_SELL: &str = "SELL";
const TIME_IN_FORCE_GTC: &str = "GTC";

struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub order_side: String,
    pub order_type: String,
    pub time_in_force: String,
}

impl Binance {
    // Account Information
    pub async fn get_account(&self) -> Result<AccountInformation> {
        Ok(self
            .transport
            .signed_get::<_, ()>(Version::V3, "/account", None)
            .await?)
    }

    // Balance for ONE Asset
    pub async fn get_balance(&self, asset: &str) -> Result<Balance> {
        let asset = asset.to_string().to_uppercase();
        self.get_account()
            .await?
            .balances
            .into_iter()
            .find(|balance| balance.asset == asset)
            .ok_or_else(|| Error::AssetsNotFound.into())
    }

    // Current open orders for ONE symbol
    pub async fn get_open_orders(&self, symbol: &str) -> Result<Vec<Order>> {
        let params = json! {{"symbol": symbol.to_uppercase()}};
        let orders = self
            .transport
            .signed_get(Version::V3, "/openOrders", Some(params))
            .await?;
        Ok(orders)
    }

    // All current open orders
    pub async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        let orders = self
            .transport
            .signed_get::<_, ()>(Version::V3, "/openOrders", None)
            .await?;
        Ok(orders)
    }

    // Check an order's status
    pub async fn order_status(&self, symbol: &str, order_id: u64) -> Result<Order> {
        let params = json! {{"symbol": symbol.to_uppercase(), "orderId": order_id}};
        Ok(self
            .transport
            .signed_get(Version::V3, "/order", Some(params))
            .await?)
    }

    // Place a LIMIT order - BUY
    pub async fn limit_buy(&self, symbol: &str, qty: f64, price: f64) -> Result<Transaction> {
        let order = OrderRequest {
            symbol: symbol.into(),
            qty,
            price,
            order_side: ORDER_SIDE_BUY.to_string(),
            order_type: ORDER_TYPE_LIMIT.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = Self::build_order(order);

        let transaction = self
            .transport
            .signed_post(Version::V3, "/order", Some(params))
            .await?;

        Ok(transaction)
    }

    // Place a LIMIT order - SELL
    pub async fn limit_sell(&self, symbol: &str, qty: f64, price: f64) -> Result<Transaction> {
        let order = OrderRequest {
            symbol: symbol.into(),
            qty,
            price,
            order_side: ORDER_SIDE_SELL.to_string(),
            order_type: ORDER_TYPE_LIMIT.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = Self::build_order(order);
        let transaction = self
            .transport
            .signed_post(Version::V3, "/order", Some(params))
            .await?;

        Ok(transaction)
    }

    // Place a MARKET order - BUY
    pub async fn market_buy(&self, symbol: &str, qty: f64) -> Result<Transaction> {
        let order = OrderRequest {
            symbol: symbol.into(),
            qty,
            price: 0.0,
            order_side: ORDER_SIDE_BUY.to_string(),
            order_type: ORDER_TYPE_MARKET.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = Self::build_order(order);
        let transaction = self
            .transport
            .signed_post(Version::V3, "/order", Some(params))
            .await?;

        Ok(transaction)
    }

    // Place a MARKET order - SELL
    pub async fn market_sell(&self, symbol: &str, qty: f64) -> Result<Transaction> {
        let order = OrderRequest {
            symbol: symbol.into(),
            qty,
            price: 0.0,
            order_side: ORDER_SIDE_SELL.to_string(),
            order_type: ORDER_TYPE_MARKET.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = Self::build_order(order);
        let transaction = self
            .transport
            .signed_post(Version::V3, "/order", Some(params))
            .await?;
        Ok(transaction)
    }

    // Check an order's status
    pub async fn cancel_order(&self, symbol: &str, order_id: u64) -> Result<OrderCanceled> {
        let params = json! {{"symbol":symbol, "orderId":order_id}};
        let order_canceled = self
            .transport
            .signed_delete(Version::V3, "/order", Some(params))
            .await?;
        Ok(order_canceled)
    }

    // Trade history
    pub async fn trade_history(&self, symbol: &str) -> Result<Vec<TradeHistory>> {
        let params = json! {{"symbol":symbol.to_uppercase()}};
        let trade_history = self
            .transport
            .signed_get(Version::V3, "/myTrades", Some(params))
            .await?;
        Ok(trade_history)
    }

    fn build_order(order: OrderRequest) -> HashMap<&'static str, String> {
        let mut params: HashMap<&str, String> = maplit::hashmap! {
            "symbol" => order.symbol,
            "side" => order.order_side,
            "type" => order.order_type,
            "quantity" => order.qty.to_string(),
        };

        if order.price != 0.0 {
            params.insert("price", order.price.to_string());
            params.insert("timeInForce", order.time_in_force.to_string());
        }
        params
    }
}

#[cfg(test)]
mod test {
    use crate::tests::test::setup;
    use anyhow::Result;

    #[tokio::test]
    async fn test_get_account() -> Result<()> {
        let b = setup()?;
        b.get_account().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_balance() -> Result<()> {
        let b = setup()?;
        b.get_balance("btc").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_open_orders() -> Result<()> {
        let b = setup()?;
        b.get_open_orders("btcusdt").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_open_orders() -> Result<()> {
        let b = setup()?;
        b.get_all_open_orders().await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_order_status() -> Result<()> {
        let b = setup()?;
        b.order_status("btcusdt", 1).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_trade_history() -> Result<()> {
        let b = setup()?;
        b.trade_history("btcusdt").await?;
        Ok(())
    }
}
