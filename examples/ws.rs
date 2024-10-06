use anyhow::Result;
use binance_async::model::websocket::{BinanceWebsocketMessage, Subscription};
use binance_async::BinanceWebsocket;
use chrono::Local;
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();

    println!("Data Stream Started ...");

    let mut ws = BinanceWebsocket::default();

    for sub in vec![
        // Subscription::Ticker("btcusdt.to_string()),
        // Subscription::AggregateTrade("btcusdt.to_string()),
        // Subscription::Candlestick("btcusdt".to_string(), "1m".to_string()),
        // Subscription::Depth("btcusdt".to_string()),
        // Subscription::MiniTicker("btcusdt".to_string()),
        // Subscription::OrderBook("btcusdt".to_string(), 10),
        Subscription::Trade("btcusdt".to_string()),
        // Subscription::UserData(listen_key),
        // Subscription::MiniTickerAll,
        // Subscription::TickerAll,
    ] {
        ws.subscribe(&sub).await?;
    }

    while let Some(msg) = ws.try_next().await? {
        // println!("\n\n{:#?}", msg)

        match msg {
            BinanceWebsocketMessage::Trade(trade) => println!("{:?}", trade),
            BinanceWebsocketMessage::Ping => println!("{:?}: {:?}", Local::now(), msg),
            _ => {}
        };
    }

    Ok(())
}
