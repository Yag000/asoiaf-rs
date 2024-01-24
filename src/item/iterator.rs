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

// This module should be refactored, but it will be done in the future.

/**
 * Iterates over the pages of a book request.
 */
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

    /**
     * Returns the next page of books.
     *
     * # Errors
     *
     * If there are no more pages to be found, it will return an Error::NoMorePages.
     * If there is an error with the request, it will return an Error::RequestError.
     */
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

/**
 * Iterates over the pages of a house request.
 */
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

    /**
     * Returns the next page of houses.
     *
     * # Errors
     *
     * If there are no more pages to be found, it will return an Error::NoMorePages.
     * If there is an error with the request, it will return an Error::RequestError.
     */
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

/**
 * Iterates over the pages of a character request.
 */
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

    /**
     * Returns the next page of characters.
     *
     * # Errors
     *
     * If there are no more pages to be found, it will return an Error::NoMorePages.
     * If there is an error with the request, it will return an Error::RequestError.
     */
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
