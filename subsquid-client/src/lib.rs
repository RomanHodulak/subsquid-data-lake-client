//! # Subsquid Client
//!
//! An asynchronous client library for reading data from [Subsquid's Data Lake API](https://docs.subsquid.io/).
//!
//! It consists of a set of traits and their implementations that provide a range of functionality that ensures
//! appropriate usage of the API. The design is defensive and expressive with comfortable user experience in mind
//! requiring no prior domain knowledge.
//!
//! The [`ClientBuilder`] struct is the first one to interact with. It provides several options to configure the client
//! along with a [`build`] method. Calling the [`build`] method fetches you a new instance of [`Client`] trait
//! implementation. Having it return a trait implementation gives the freedom and flexibility to swap its implementation
//! from the library side as well as the user side, allowing you to write code that is testable and backwards compatible.
//!
//! The reponses of the API are decoded into strongly typed Rust data structures, having [`Block`] at the root.
//!
//! [`ClientBuilder`]: crate::ClientBuilder
//! [`build`]: crate::ClientBuilder::build
//! [`Block`]: crate::Block

mod builder;
mod client;
mod http;
mod tokio;

pub use builder::*;
pub use client::*;
pub use http::*;
