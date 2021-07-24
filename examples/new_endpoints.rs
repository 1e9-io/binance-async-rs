use crate::binance::Binance;
use binance_async as binance;
use failure::Fallible;
use std::env::var;

#[tokio::main]
async fn main() -> Fallible<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();

    let api_key = var("BINANCE_KEY")?;
    let secret_key = var("BINANCE_SECRET")?;

    let bn = Binance::with_credential(&api_key, &secret_key);

    // get aggregated historical trades
    match bn.get_historical_trades(
        "BTCUSDT",
        10, 963563573)?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

