#[cfg(test)]
pub mod test {
    use crate::Binance;
    use anyhow::Result;

    pub fn setup() -> Result<Binance> {
        dotenv::dotenv().ok();
        let _ = env_logger::builder().is_test(true).try_init();
        Ok(Binance::with_credential(
            &std::env::var("BINANCE_KEY")?,
            &std::env::var("BINANCE_SECRET")?,
        ))
    }
}
