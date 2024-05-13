//! # Subsquid CLI
//!
//! A CLI tool that continuously streams block data fetched from [Subsquid's Data Lake API](https://docs.subsquid.io/)
//! to standard output.
//!
//! The output is printed to stdout and may be used for downstream decoding or piping to external sources such as
//! [Apache Kafka](https://kafka.apache.org/).
use log::{LevelFilter, SetLoggerError};
use simple_logger::SimpleLogger;
use std::str::FromStr;
use subsquid_client::{Client, ClientBuilder, DataSource, StartHeight};
use url::Url;

#[tokio::main]
async fn main() {
    let _ = init_logger();

    let url = Url::from_str("https://v2.archive.subsquid.io/network/ethereum-mainnet").unwrap();

    let mut client = ClientBuilder::new()
        .with_data_source(DataSource::Subsquid(url))
        .build();

    let (sender, mut receiver) = tokio::sync::mpsc::channel(1024);

    tokio::spawn(async move {
        if let Err(error) = client.stream(sender, StartHeight::Latest).await {
            eprintln!("{error}");
        }
    });

    while let Some(block) = receiver.recv().await {
        print!("{block:#?}");
    }
}

fn init_logger() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger::new()))
        .map(|()| log::set_max_level(LevelFilter::Debug))
}
