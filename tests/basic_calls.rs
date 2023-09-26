use asoiaf_api::client::Client;

#[tokio::test]
async fn test_get_books() {
    let client = Client::new();
    let books = client.get_books().await;
    assert!(books.is_ok());
    let books = books.unwrap();
    assert!(!books.is_empty());
}

#[tokio::test]
async fn test_get_characters() {
    let client = Client::new();
    let characters = client.get_characters().await;
    assert!(characters.is_ok());
    let characters = characters.unwrap();
    assert!(!characters.is_empty());
}

#[tokio::test]
async fn test_get_houses() {
    let client = Client::new();
    let houses = client.get_houses().await;
    println!("{:?}", houses);
    assert!(houses.is_ok());
    assert!(!houses.unwrap().is_empty());
}
