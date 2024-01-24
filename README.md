# asoiaf-rs

A Rust library for interacting with the [An API of Ice and Fire](https://anapioficeandfire.com/).

## Usage

There is more documentation available on the [docs.rs](https://docs.rs/asoiaf_api/latest/asoiaf_api/) page.

```rust
use asoiaf_api::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let books = client.get_books().await.unwrap();
    let characters = client.get_characters().await.unwrap();
    let houses = client.get_houses().await.unwrap();
}
```

This minimal example will fetch all books, characters and houses from the API.

## License

This project is licensed under the [MIT license](LICENSE-MIT) and [Apache License 2.0](LICENSE-APACHE).

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `asoiaf-rs` by you, shall be licensed as MIT and Apache 2.0,
without any additional terms or conditions.

Having said that, every contribution is welcome and I look forward to your PRs and issues.
