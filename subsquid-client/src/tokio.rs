use crate::{Block, SendBlockAsync, SendError};
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

#[async_trait]
impl SendBlockAsync for Sender<Block> {
    async fn send(&self, block: Block) -> Result<(), SendError> {
        Ok(self
            .send(block)
            .await
            .map_err(|error| SendError(Box::new(error)))?)
    }
}
