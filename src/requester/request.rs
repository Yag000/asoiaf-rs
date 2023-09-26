use super::{pagination::Pagination, ToRequest, ToUrl};

/// Struct that is used to make book requests to the API.
#[derive(Default)]
pub(crate) struct BookRequest {
    pagination: Option<Pagination>,
}

impl ToRequest for BookRequest {
    fn to_request(&self) -> String {
        let mut url = String::from("books");
        if let Some(pagination) = &self.pagination {
            url.push('?');
            url.push_str(&pagination.to_url());
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
}

#[derive(Default)]
pub(crate) struct CharacterRequest {
    pagination: Option<Pagination>,
}

impl ToRequest for CharacterRequest {
    fn to_request(&self) -> String {
        let mut url = String::from("characters");
        if let Some(pagination) = &self.pagination {
            url.push('?');
            url.push_str(&pagination.to_url());
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
}

#[derive(Default)]
pub(crate) struct HouseRequest {
    pagination: Option<Pagination>,
}

impl ToRequest for HouseRequest {
    fn to_request(&self) -> String {
        let mut url = String::from("houses");
        if let Some(pagination) = &self.pagination {
            url.push('?');
            url.push_str(&pagination.to_url());
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
}
