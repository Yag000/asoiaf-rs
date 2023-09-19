use crate::requester::ToUrl;

pub struct Pagination {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

impl ToUrl for Pagination {
    fn to_url(&self) -> String {
        let mut url = String::new();
        if let Some(page) = self.page {
            url.push_str(&format!("?page={}", page));
        }
        if let Some(page_size) = self.page_size {
            url.push_str(&format!("?pageSize={}", page_size));
        }
        url
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: None,
            page_size: None,
        }
    }
}

impl Pagination {
    pub fn new(page: u32, page_size: u32) -> Self {
        Self {
            page: Some(page),
            page_size: Some(page_size),
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination() {
        let pagination = Pagination::new(1, 10);
        assert_eq!(pagination.to_url(), "?page=1?pageSize=10");
    }

    #[test]
    fn test_missing_page() {
        let pagination = Pagination::default().page_size(10);
        assert_eq!(pagination.to_url(), "?pageSize=10");
    }

    #[test]
    fn test_missing_page_size() {
        let pagination = Pagination::default().page(1);
        assert_eq!(pagination.to_url(), "?page=1");
    }

    #[test]
    fn test_no_pagination() {
        let pagination = Pagination::default();
        assert_eq!(pagination.to_url(), "");
    }
}
