use asoiaf_api::{
    client::Client,
    error::Error,
    item::{Book, Character, House},
    requester::filter::{BookFilter, CharacterFilter, HouseFilter},
};

static MAX_SIZE: usize = 50;

async fn collect_books(book_filter: BookFilter) -> Result<Vec<Book>, Error> {
    let client = Client::new();

    let mut characters = client.get_book_filter_iterator(book_filter, MAX_SIZE);

    let mut vect = Vec::new();
    loop {
        match characters.next().await {
            Ok(mut character) => vect.append(&mut character),
            Err(Error::NoMorePages) => break,
            Err(e) => return Err(e),
        }
    }

    Ok(vect)
}

#[tokio::test]
async fn test_book_filter_name() {
    let books = collect_books(BookFilter::Name("A Game of Thrones".to_string()))
        .await
        .unwrap();

    assert_eq!(books.len(), 1);
}

#[tokio::test]
async fn test_book_filter_from_release_date() {
    let books = collect_books(BookFilter::FromReleaseDate("2010-08-01".to_string()))
        .await
        .unwrap();

    assert_eq!(books.len(), 6);
}

#[tokio::test]
async fn test_book_filter_to_release_date() {
    let books = collect_books(BookFilter::ToReleaseDate("2010-08-01".to_string()))
        .await
        .unwrap();

    assert_eq!(books.len(), 6);
}

async fn collect_characters(character_filter: CharacterFilter) -> Result<Vec<Character>, Error> {
    let client = Client::new();

    let mut characters = client.get_character_filter_iterator(character_filter, MAX_SIZE);

    let mut vect = Vec::new();
    loop {
        match characters.next().await {
            Ok(mut character) => vect.append(&mut character),
            Err(Error::NoMorePages) => break,
            Err(e) => return Err(e),
        }
    }

    Ok(vect)
}

#[tokio::test]
async fn test_character_filter_name() {
    let characters = collect_characters(CharacterFilter::Name("Jon Snow".to_string()))
        .await
        .unwrap();

    assert_eq!(characters.len(), 1);
}

#[tokio::test]
async fn test_character_filter_born() {
    let characters = collect_characters(CharacterFilter::Born("In 283 AC".to_string()))
        .await
        .unwrap();
    assert!(!characters.is_empty());
}

#[tokio::test]
async fn test_character_filter_died() {
    let characters = collect_characters(CharacterFilter::Died(
        "In 299 AC, at Castle Black".to_string(),
    ))
    .await
    .unwrap();

    assert!(!characters.is_empty());
}

#[tokio::test]
async fn test_character_filter_gender() {
    let characters = collect_characters(CharacterFilter::Gender("Male".to_string()))
        .await
        .unwrap();

    assert!(!characters.is_empty());
}

#[tokio::test]
async fn test_character_filter_culture() {
    let characters = collect_characters(CharacterFilter::Culture("Valemen".to_string()))
        .await
        .unwrap();

    assert!(!characters.is_empty());
}

#[tokio::test]
async fn test_character_filter_is_alive() {
    let characters = collect_characters(CharacterFilter::IsAlive(true))
        .await
        .unwrap();

    assert!(!characters.is_empty());
}

async fn collect_houses(house_filter: HouseFilter) -> Result<Vec<House>, Error> {
    let client = Client::new();

    let mut houses = client.get_house_filter_iterator(house_filter, MAX_SIZE);

    let mut vect = Vec::new();
    loop {
        match houses.next().await {
            Ok(mut house) => vect.append(&mut house),
            Err(Error::NoMorePages) => break,
            Err(e) => return Err(e),
        }
    }

    Ok(vect)
}

#[tokio::test]
async fn test_house_filter_name() {
    let houses = collect_houses(HouseFilter::Name("House Stark of Winterfell".to_string()))
        .await
        .unwrap();

    assert_eq!(houses.len(), 1);
}

#[tokio::test]
async fn test_house_filter_region() {
    let houses = collect_houses(HouseFilter::Region("The North".to_string()))
        .await
        .unwrap();

    assert!(!houses.is_empty());
}

#[tokio::test]
async fn test_house_filter_words() {
    let houses = collect_houses(HouseFilter::Words("Winter is Coming".to_string()))
        .await
        .unwrap();

    println!("{:?}", houses);
    assert_eq!(houses.len(), 1);
}

#[tokio::test]
async fn test_house_filter_has_words() {
    let houses = collect_houses(HouseFilter::HasWords(true)).await.unwrap();

    assert!(!houses.is_empty());
}

#[tokio::test]
async fn test_house_filter_has_died_out() {
    let houses = collect_houses(HouseFilter::HasDiedOut(true)).await.unwrap();

    assert!(!houses.is_empty());
}

#[tokio::test]
async fn test_house_filter_has_an_ancestral_weapon() {
    let houses = collect_houses(HouseFilter::HasAncestralWeapons(true))
        .await
        .unwrap();

    assert!(!houses.is_empty());
}

#[tokio::test]
async fn test_house_filter_has_a_seat() {
    let houses = collect_houses(HouseFilter::HasSeats(true)).await.unwrap();

    assert!(!houses.is_empty());
}

#[tokio::test]
async fn test_house_filter_has_a_current_lord() {
    let houses = collect_houses(HouseFilter::HasTitles(true)).await.unwrap();

    assert!(!houses.is_empty());
}
