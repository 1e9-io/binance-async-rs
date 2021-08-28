use crate::error::Error;
use crate::{
    client::Binance,
    model::{
        AccountInformation, AssetDetail, Balance, DepositAddressData, DepositHistory, Order,
        OrderCanceled, TradeHistory, Transaction,
    },
};
use chrono::prelude::*;
use failure::Fallible;
use serde_json::json;
use std::collections::HashMap;

const ORDER_TYPE_LIMIT: &str = "LIMIT";
const ORDER_TYPE_MARKET: &str = "MARKET";
const ORDER_SIDE_BUY: &str = "BUY";
const ORDER_SIDE_SELL: &str = "SELL";
const TIME_IN_FORCE_GTC: &str = "GTC";

const API_V3_ORDER: &str = "/api/v3/order";

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
    pub async fn get_account(&self) -> Fallible<AccountInformation> {
        let account_info = self
            .transport
            .signed_get::<_, ()>("/api/v3/account", None)
            .await?;
        Ok(account_info)
    }

    // Balance for ONE Asset
    pub async fn get_balance(&self, asset: &str) -> Fallible<Balance> {
        let asset = asset.to_string();
        self.get_account()
            .await?
            .balances
            .into_iter()
            .find(|balance| balance.asset == asset)
            .ok_or_else(|| Error::AssetsNotFound.into())
    }

    // Current open orders for ONE symbol
    pub async fn get_open_orders(&self, symbol: &str) -> Fallible<Vec<Order>> {
        let params = json! {{"symbol": symbol}};
        let orders = self
            .transport
            .signed_get("/api/v3/openOrders", Some(params))
            .await?;
        Ok(orders)
    }

    // All current open orders
    pub async fn get_all_open_orders(&self) -> Fallible<Vec<Order>> {
        let orders = self
            .transport
            .signed_get::<_, ()>("/api/v3/openOrders", None)
            .await?;
        Ok(orders)
    }

    // Check an order's status
    pub async fn order_status(&self, symbol: &str, order_id: u64) -> Fallible<Order> {
        let params = json! {{"symbol": symbol, "orderId": order_id}};

        let order = self
            .transport
            .signed_get(API_V3_ORDER, Some(params))
            .await?;
        Ok(order)
    }

    // Place a LIMIT order - BUY
    pub async fn limit_buy(&self, symbol: &str, qty: f64, price: f64) -> Fallible<Transaction> {
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
            .signed_post(API_V3_ORDER, Some(params))
            .await?;

        Ok(transaction)
    }

    // Place a LIMIT order - SELL
    pub async fn limit_sell(&self, symbol: &str, qty: f64, price: f64) -> Fallible<Transaction> {
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
            .signed_post(API_V3_ORDER, Some(params))
            .await?;

        Ok(transaction)
    }

    // Place a MARKET order - BUY
    pub async fn market_buy(&self, symbol: &str, qty: f64) -> Fallible<Transaction> {
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
            .signed_post(API_V3_ORDER, Some(params))
            .await?;

        Ok(transaction)
    }

    // Place a MARKET order - SELL
    pub async fn market_sell(&self, symbol: &str, qty: f64) -> Fallible<Transaction> {
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
            .signed_post(API_V3_ORDER, Some(params))
            .await?;
        Ok(transaction)
    }

    // Check an order's status
    pub async fn cancel_order(&self, symbol: &str, order_id: u64) -> Fallible<OrderCanceled> {
        let params = json! {{"symbol":symbol, "orderId":order_id}};
        let order_canceled = self
            .transport
            .signed_delete(API_V3_ORDER, Some(params))
            .await?;
        Ok(order_canceled)
    }

    // Trade history
    pub async fn trade_history(&self, symbol: &str) -> Fallible<Vec<TradeHistory>> {
        let params = json! {{"symbol":symbol}};
        let trade_history = self
            .transport
            .signed_get("/api/v3/myTrades", Some(params))
            .await?;
        Ok(trade_history)
    }

    pub async fn get_deposit_address(&self, symbol: &str) -> Fallible<DepositAddressData> {
        let params = json! {{"asset":symbol}};
        let deposit_address = self
            .transport
            .signed_get("/wapi/v3/depositAddress.html", Some(params))
            .await?;
        Ok(deposit_address)
    }

    pub async fn get_deposit_history(
        &self,
        symbol: Option<&str>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Fallible<DepositHistory> {
        let params = json! {{"asset":symbol, "startTime":start_time.map(|t| t.timestamp_millis()), "endTime":end_time.map(|t| t.timestamp_millis())}};
        let deposit_history = self
            .transport
            .signed_get("/wapi/v3/depositHistory.html", Some(params))
            .await?;
        Ok(deposit_history)
    }

    pub async fn asset_detail(&self) -> Fallible<AssetDetail> {
        let asset_detail = self
            .transport
            .signed_get::<_, ()>("/wapi/v3/assetDetail.html", None)
            .await?;
        Ok(asset_detail)
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
