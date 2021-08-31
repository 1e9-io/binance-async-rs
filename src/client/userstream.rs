use crate::{
    client::Binance,
    model::{Success, UserDataStream},
};
use failure::Fallible;

const USER_DATA_STREAM: &str = "/api/v1/userDataStream";

impl Binance {
    // User Stream
    pub async fn user_stream_start(&self) -> Fallible<UserDataStream> {
        Ok(self.transport.post::<_, ()>(USER_DATA_STREAM, None).await?)
    }

    // Current open orders on a symbol
    pub async fn user_stream_keep_alive(&self, listen_key: &str) -> Fallible<Success> {
        Ok(self
            .transport
            .put(
                USER_DATA_STREAM,
                Some(vec![("listen_key", listen_key.to_string())]),
            )
            .await?)
    }

    pub async fn user_stream_close(&self, listen_key: &str) -> Fallible<Success> {
        let success = self
            .transport
            .delete(
                USER_DATA_STREAM,
                Some(vec![("listen_key", listen_key.to_string())]),
            )
            .await?;
        Ok(success)
    }
}
