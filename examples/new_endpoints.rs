use crate::binance::Binance;
use anyhow::Result;
use binance_async as binance;
use std::env::var;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = var("BINANCE_KEY")?;
    let secret_key = var("BINANCE_SECRET")?;

    let bn = Binance::with_credential(&api_key, &secret_key);

    match bn.get_historical_trades("BTCUSDT", 10, 963563573).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
