use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use thiserror::Error;

/// Errors related to client-server API communication.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Failed to perform HTTP request")]
    HttpRequest(#[from] reqwest::Error),
    #[error("Failed to decode response as UTF-8 string")]
    Utf8Decoding(#[from] FromUtf8Error),
    #[error("Failed to parse number")]
    IntegerDecoding(#[from] ParseIntError),
    #[error("Failed to parse URL")]
    UrlDecoding(#[from] url::ParseError),
    #[error("Failed to parse JSON")]
    JsonDecoding(#[from] serde_json::Error),
    #[error("Failed to send block")]
    SenderError(#[from] SendError),
}

/// Error for failed channel send.
#[derive(Debug, Error)]
#[error("{0}")]
pub struct SendError(#[from] pub(crate) Box<dyn Error>);

/// The `SendBlockAsync` trait allows for sending blocks asynchronously.
///
/// Implementors of this trait are called `senders`.
///
/// Senders are defined by only one method, `send`. Senders usually come from channels, paired with one or multiple
/// receivers. In this situation, each call to `send` notifies receivers and retrives the last sent value.
#[async_trait]
pub trait SendBlockAsync: Send {
    /// Sends the block for processing defined by the implementor.
    async fn send(&self, block: Block) -> Result<(), SendError>;
}

/// The `StartHeight` defines several variants expressing how the first processed block height should be calculated.
#[derive(Debug)]
pub enum StartHeight {
    /// Using this variant
    Latest,
    /// Using this variant
    Since(u64),
}

/// The `DataLakeClient` trait allows for reading data from the data lake API.
///
/// It comes with one method, `stream`.
#[async_trait]
pub trait Client {
    /// Starts streaming block data since `start_height`.
    ///
    /// Each [`Block`] is streamed via
    async fn stream(
        &mut self,
        sender: impl SendBlockAsync,
        start_height: StartHeight,
    ) -> Result<(), ClientError>;
}

/// The `Block` struct describes block data retrived by the Data Lake API.
///
/// It is the outer-most data structure in the response.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Block {
    header: Header,
}

/// Data-structure containing block header data for block of height defined in `number` field.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    number: u64,
    hash: String,
    parent_hash: String,
}
