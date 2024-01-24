//! # asoiaf-rs
//!
//! This is a library for interacting with the [An API of Ice and Fire](https://anapioficeandfire.com/).
//! # Examples
//!
//! ## Basic calls
//!
//! ```rust
//! use asoiaf_api::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new();
//!
//!     let books = client.get_books().await.unwrap();
//!     let characters = client.get_characters().await.unwrap();
//!     let houses = client.get_houses().await.unwrap();
//! }
//! ```
//!
//! ## Iterators
//!
//!```rust
//! use asoiaf_api::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     // We specify a maximum size of 50 items per page
//!     let mut iterator = Client::new().get_character_iterator(50);
//!
//!     // We iterate over all the characters
//!     while let Ok(result) = iterator.next().await {
//!         println!("{:?}", result);
//!     }
//! }
//! ```
//!
//! ## Filters
//!
//! ```rust
//!
//! use asoiaf_api::{Client, CharacterFilter};
//!
//! #[tokio::main]
//! async fn main() {
//!     // We filter by culture
//!     let character_filter = CharacterFilter::Culture("Northmen".to_string());
//!     // We specify a maximum size of 50 items per page
//!     let iterator = Client::new().get_character_filter_iterator(character_filter, 50);
//! }
//! ```
//!
//! # Features
//!
//! [`Client`](client::Client) is the main struct of this library. It is used to make requests to the API.
//! We can filter the results of the requests by using the [`Filter`](requester::filter) structs.
//! We can also iterate over the results of the requests by using the [`Iterator`](item::iterator) structs.

pub mod client;
pub mod error;
pub mod item;
pub mod requester;

pub use item::{Book, Character, House, Items};
pub use requester::filter::{BookFilter, CharacterFilter, HouseFilter};
pub use requester::pagination::Pagination;

pub use item::iterator::{BookIterator, CharacterIterator, HouseIterator};

pub use client::Client;
pub use error::Error;
