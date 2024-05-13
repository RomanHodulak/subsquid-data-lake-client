use crate::{Client, HttpClient};
use std::marker::PhantomData;
use url::Url;

/// A struct that marks the builder as without the ability to build Client due to missing data source.
#[derive(Debug)]
pub struct DataSourceMissing;

/// A struct that marks the builder as possesing the ability to build Client while having data source.
#[derive(Debug)]
pub struct DataSourceSet;

/// Data source to connect the Data Lake client to.
#[derive(Debug, Clone, PartialEq)]
pub enum DataSource {
    /// This variant treats the [`Url`] as standard subsquid API.
    Subsquid(Url),
}

/// This struct
#[derive(Debug)]
pub struct ClientBuilder<State> {
    _phantom_data: PhantomData<State>,
    data_source: Option<DataSource>,
}

impl ClientBuilder<DataSourceMissing> {
    pub fn new() -> Self {
        Self {
            data_source: None,
            _phantom_data: PhantomData,
        }
    }

    /// The created client will use the `data_source` to connect to and fetch the data from.
    pub fn with_data_source(mut self, data_source: DataSource) -> ClientBuilder<DataSourceSet> {
        self.data_source.replace(data_source);

        ClientBuilder {
            _phantom_data: PhantomData,
            data_source: self.data_source,
        }
    }
}

impl ClientBuilder<DataSourceSet> {
    /// Creates the [`Client`] appropriate for the options set on the builder.
    pub fn build(self) -> impl Client {
        HttpClient::new(match self.data_source
            .expect("The data_source must be set, otherwise this wouldn't be ClientBuilder<DataSourceSet>") {
            DataSource::Subsquid(url) => url,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_using_data_source_in_builder_posses_expected_data_source() {
        let url = Url::parse("http://example.org").expect("Can't fail on correctly formatted URL");
        let data_source = DataSource::Subsquid(url);

        let builder = ClientBuilder::new().with_data_source(data_source.clone());

        let expected_data_source = Some(data_source);
        let actual_data_source = builder.data_source.clone();

        assert_eq!(expected_data_source, actual_data_source);
    }

    #[test]
    fn test_using_data_source_in_builder_does_not_panic() {
        let url = Url::parse("http://example.org").expect("Can't fail on correctly formatted URL");
        let data_source = DataSource::Subsquid(url);

        let builder = ClientBuilder::new().with_data_source(data_source);

        // The builder must not panic.
        let _ = builder.build();
    }
}
