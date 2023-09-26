use asoiaf_api::client::Client;

#[tokio::test]
async fn test_book_iterator() {
    // Really slow iterator to make sure that it receives everything
    let mut iterator = Client::new().get_book_iterator(1);

    let mut total_books = 0;

    while let Ok(result) = iterator.next().await {
        total_books += result.len();
    }

    assert_eq!(total_books, 12);
}

#[tokio::test]
async fn test_house_iterator() {
    let mut iterator = Client::new().get_house_iterator(50);

    let mut total_houses = 0;

    while let Ok(result) = iterator.next().await {
        total_houses += result.len();
    }

    assert_eq!(total_houses, 444);
}

#[tokio::test]
async fn test_character_iterator() {
    let mut iterator = Client::new().get_character_iterator(50);

    let mut total_characters = 0;

    while let Ok(result) = iterator.next().await {
        total_characters += result.len();
    }

    assert_eq!(total_characters, 2134);
}
