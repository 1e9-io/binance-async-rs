use anyhow::Result;
use binance_async::Binance;

#[tokio::test]
async fn ping() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();

    let binance = Binance::new();

    binance.ping().await?;

    Ok(())
}
