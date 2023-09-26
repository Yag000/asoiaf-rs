use crate::requester::ToUrl;

#[derive(Debug, Clone, Default)]
pub struct Pagination {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

impl ToUrl for Pagination {
    fn to_url(&self) -> String {
        let mut url = String::new();
        if let Some(page) = self.page {
            url.push_str(&format!("page={}", page));
        }
        if let Some(page_size) = self.page_size {
            if !url.is_empty() {
                url.push('&');
            }
            url.push_str(&format!("pageSize={}", page_size));
        }
        url
    }
}

impl Pagination {
    pub fn new(page: usize, page_size: usize) -> Self {
        Self {
            page: Some(page),
            page_size: Some(page_size),
        }
    }

    pub fn page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }

    pub fn page_size(mut self, page_size: usize) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn next_page(&mut self) {
        eprintln!("{:?}", self.page);
        if let Some(page) = &mut self.page {
            *page += 1;
        }
        eprintln!("{:?}", self.page);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination() {
        let pagination = Pagination::new(1, 10);
        assert_eq!(pagination.to_url(), "page=1&pageSize=10");
    }

    #[test]
    fn test_missing_page() {
        let pagination = Pagination::default().page_size(10);
        assert_eq!(pagination.to_url(), "pageSize=10");
    }

    #[test]
    fn test_missing_page_size() {
        let pagination = Pagination::default().page(1);
        assert_eq!(pagination.to_url(), "page=1");
    }

    #[test]
    fn test_no_pagination() {
        let pagination = Pagination::default();
        assert_eq!(pagination.to_url(), "");
    }

    #[test]
    fn test_next_page() {
        let mut pagination = Pagination::default().page(1);
        assert_eq!(pagination.to_url(), "page=1");
        pagination.next_page();
        assert_eq!(pagination.to_url(), "page=2");
    }
}
