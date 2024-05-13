use crate::{Block, Client, ClientError, SendBlockAsync, StartHeight};
use async_trait::async_trait;
use std::str::FromStr;
use std::time::Duration;
use url::Url;

#[derive(Debug)]
pub struct HttpClient {
    url: Url,
}

impl HttpClient {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    async fn worker_url_for_height(&mut self, height: u64) -> Result<String, ClientError> {
        let url = self.worker_determiner_url(height)?;
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?.to_vec();
        let worker_url = String::from_utf8(bytes)?;

        Ok(worker_url)
    }

    async fn latest_height(&mut self) -> Result<u64, ClientError> {
        let url = self.latest_height_url()?;
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?.to_vec();
        let height = String::from_utf8(bytes)?;
        let height = u64::from_str(&height)?;

        Ok(height)
    }

    fn latest_height_url(&mut self) -> Result<Url, ClientError> {
        Ok(Url::parse(&format!("{}/height", self.url))?)
    }

    fn worker_determiner_url(&mut self, height: u64) -> Result<Url, ClientError> {
        Ok(Url::parse(&format!("{}/{height}/worker", self.url))?)
    }
}

#[async_trait]
impl Client for HttpClient {
    async fn stream(
        &mut self,
        sender: impl SendBlockAsync,
        start_height: StartHeight,
    ) -> Result<(), ClientError> {
        let mut latest_unread_height: Option<u64> = None;
        let client = reqwest::Client::new();

        loop {
            let last_height = self.latest_height().await?;
            let start_height = match latest_unread_height {
                Some(height) => height,
                None => match start_height {
                    StartHeight::Latest => last_height,
                    StartHeight::Since(height) => height,
                },
            };

            for height in start_height..=last_height {
                let worker_url = self.worker_url_for_height(height).await?;

                let response = client
                    .post(worker_url)
                    .header("content-type", "application/json")
                    .header("accept", "application/json")
                    .body(format!(
                        r#"{{
                        "fromBlock": {height},
                        "toBlock": {height},
                        "fields": {{
                            "transaction": {{ "hash": true }},
                            "log": {{}}
                        }},
                        "transactions":[],
                        "logs": [],
                        "includeAllBlocks": true
                    }}"#
                    ))
                    .send()
                    .await?;

                let data = response.bytes().await?;
                let data: Vec<Block> = serde_json::from_slice(&data[..])?;

                for block in data {
                    sender.send(block).await?;
                }
            }

            latest_unread_height.replace(last_height + 1);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
