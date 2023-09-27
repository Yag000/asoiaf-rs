use super::{
    filter::{BookFilter, CharacterFilter, HouseFilter},
    pagination::Pagination,
    ToRequest, ToUrl,
};

/// Struct that is used to make book requests to the API.
#[derive(Default)]
pub(crate) struct BookRequest {
    filter: Option<BookFilter>,
    pagination: Option<Pagination>,
}

impl ToRequest for BookRequest {
    fn to_request(&self) -> String {
        let mut url = String::from("books");
        let mut options = Vec::new();
        if let Some(pagination) = &self.pagination {
            options.push(pagination.to_url());
        }
        if let Some(filter) = &self.filter {
            options.push(filter.to_url());
        }

        if !options.is_empty() {
            url.push('?');
            url.push_str(&options.join("&"));
        }
        url
    }

    fn update_pagination(&mut self, pagination: Pagination) {
        self.pagination = Some(pagination);
    }

    fn next_page(&mut self) {
        if let Some(pagination) = &mut self.pagination {
            pagination.next_page();
        }
    }
}

impl BookRequest {
    pub fn pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    pub fn filter(mut self, filter: BookFilter) -> Self {
        self.filter = Some(filter);
        self
    }
}

#[derive(Default)]
pub(crate) struct CharacterRequest {
    filter: Option<CharacterFilter>,
    pagination: Option<Pagination>,
}

impl ToRequest for CharacterRequest {
    fn to_request(&self) -> String {
        let mut url = String::from("characters");

        let mut options = Vec::new();
        if let Some(pagination) = &self.pagination {
            options.push(pagination.to_url());
        }
        if let Some(filter) = &self.filter {
            options.push(filter.to_url());
        }

        if !options.is_empty() {
            url.push('?');
            url.push_str(&options.join("&"));
        }
        url
    }

    fn update_pagination(&mut self, pagination: Pagination) {
        self.pagination = Some(pagination);
    }

    fn next_page(&mut self) {
        if let Some(pagination) = &mut self.pagination {
            pagination.next_page();
        }
    }
}

impl CharacterRequest {
    pub fn pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    pub fn filter(mut self, filter: CharacterFilter) -> Self {
        self.filter = Some(filter);
        self
    }
}

#[derive(Default)]
pub(crate) struct HouseRequest {
    filter: Option<HouseFilter>,
    pagination: Option<Pagination>,
}

impl ToRequest for HouseRequest {
    fn to_request(&self) -> String {
        let mut url = String::from("houses");
        let mut options = Vec::new();
        if let Some(pagination) = &self.pagination {
            options.push(pagination.to_url());
        }
        if let Some(filter) = &self.filter {
            options.push(filter.to_url());
        }

        if !options.is_empty() {
            url.push('?');
            url.push_str(&options.join("&"));
        }
        url
    }

    fn update_pagination(&mut self, pagination: Pagination) {
        self.pagination = Some(pagination);
    }

    fn next_page(&mut self) {
        if let Some(pagination) = &mut self.pagination {
            pagination.next_page();
        }
    }
}

impl HouseRequest {
    pub fn pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    pub fn filter(mut self, filter: HouseFilter) -> Self {
        self.filter = Some(filter);
        self
    }
}
