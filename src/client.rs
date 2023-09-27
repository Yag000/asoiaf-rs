use crate::{
    error::Error,
    item::{
        iterator::{BookIterator, CharacterIterator, HouseIterator},
        Book, Character, House,
    },
    requester::{
        filter::{BookFilter, CharacterFilter, HouseFilter},
        get,
        pagination::Pagination,
        request::{BookRequest, CharacterRequest, HouseRequest},
        ToRequest,
    },
};

#[derive(Default)]
pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub(crate) async fn get_request<T>(&self, request: &impl ToRequest) -> Result<Vec<T>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let answer = get(request).await?;

        let answer = serde_json::from_str::<Vec<T>>(&answer)?;
        Ok(answer)
    }

    pub async fn get_books(&self) -> Result<Vec<Book>, Error> {
        let request = BookRequest::default();
        self.get_request(&request).await
    }

    pub fn get_book_iterator(&self, limit: usize) -> BookIterator {
        let request = BookRequest::default().pagination(Pagination::new(1, limit));
        BookIterator::new(request)
    }

    pub fn get_book_filter_iterator(&self, book_filter: BookFilter, limit: usize) -> BookIterator {
        let request = BookRequest::default()
            .pagination(Pagination::new(1, limit))
            .filter(book_filter);
        BookIterator::new(request)
    }

    pub async fn get_characters(&self) -> Result<Vec<Character>, Error> {
        let request = CharacterRequest::default().pagination(Pagination::new(1, 1000));
        self.get_request(&request).await
    }

    pub fn get_character_iterator(&self, limit: usize) -> CharacterIterator {
        let request = CharacterRequest::default().pagination(Pagination::new(1, limit));
        CharacterIterator::new(request)
    }

    pub fn get_character_filter_iterator(
        &self,
        character_filter: CharacterFilter,
        limit: usize,
    ) -> CharacterIterator {
        let request = CharacterRequest::default()
            .pagination(Pagination::new(1, limit))
            .filter(character_filter);
        CharacterIterator::new(request)
    }

    pub async fn get_houses(&self) -> Result<Vec<House>, Error> {
        let request = HouseRequest::default();
        self.get_request(&request).await
    }

    pub fn get_house_iterator(&self, limit: usize) -> HouseIterator {
        let request = HouseRequest::default().pagination(Pagination::new(1, limit));
        HouseIterator::new(request)
    }

    pub fn get_house_filter_iterator(
        &self,
        house_filter: HouseFilter,
        limit: usize,
    ) -> HouseIterator {
        let request = HouseRequest::default()
            .pagination(Pagination::new(1, limit))
            .filter(house_filter);
        HouseIterator::new(request)
    }
}
