use asoiaf_api::client::Client;

#[tokio::test]
async fn test_get_books() {
    let client = Client::new();
    let books = client.get_books().await;
    println!("{:?}", books);
    assert!(books.is_ok());
    assert!(!books.unwrap().is_empty());
}
