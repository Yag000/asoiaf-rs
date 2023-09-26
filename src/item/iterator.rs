use crate::{
    client::Client,
    error::Error,
    item::{Book, House},
    requester::{
        request::{BookRequest, CharacterRequest, HouseRequest},
        ToRequest,
    },
};

use super::Character;

pub struct BookIterator {
    client: Client,

    has_found_limit: bool,

    request: BookRequest,
}

impl BookIterator {
    pub(crate) fn new(request: BookRequest) -> Self {
        Self {
            has_found_limit: false,
            request,
            client: Client::new(),
        }
    }

    pub async fn next(&mut self) -> Result<Vec<Book>, Error> {
        if self.has_found_limit {
            return Err(Error::NoMorePages);
        }

        let answer = self.client.get_request::<Book>(&self.request).await?;

        self.request.next_page();

        if answer.is_empty() {
            self.has_found_limit = true;
            Err(Error::NoMorePages)
        } else {
            Ok(answer)
        }
    }
}

pub struct HouseIterator {
    client: Client,

    has_found_limit: bool,

    request: HouseRequest,
}

impl HouseIterator {
    pub(crate) fn new(request: HouseRequest) -> Self {
        Self {
            has_found_limit: false,
            request,
            client: Client::new(),
        }
    }

    pub async fn next(&mut self) -> Result<Vec<House>, Error> {
        if self.has_found_limit {
            return Err(Error::NoMorePages);
        }

        let answer = self.client.get_request::<House>(&self.request).await?;

        self.request.next_page();

        if answer.is_empty() {
            self.has_found_limit = true;
            Err(Error::NoMorePages)
        } else {
            Ok(answer)
        }
    }
}

pub struct CharacterIterator {
    client: Client,

    has_found_limit: bool,

    request: CharacterRequest,
}

impl CharacterIterator {
    pub(crate) fn new(request: CharacterRequest) -> Self {
        Self {
            has_found_limit: false,
            request,
            client: Client::new(),
        }
    }

    pub async fn next(&mut self) -> Result<Vec<Character>, Error> {
        if self.has_found_limit {
            return Err(Error::NoMorePages);
        }

        let answer = self.client.get_request::<Character>(&self.request).await?;

        self.request.next_page();

        if answer.is_empty() {
            self.has_found_limit = true;
            Err(Error::NoMorePages)
        } else {
            Ok(answer)
        }
    }
}
