use self::pagination::Pagination;

pub mod pagination;
pub mod request;

/// Trait that is used to convert a struct to a url.
/// It is used to make requests to the API.
pub(crate) trait ToUrl {
    fn to_url(&self) -> String;
}

pub trait ToRequest {
    fn to_request(&self) -> String;
    fn update_pagination(&mut self, pagination: Pagination);
    fn next_page(&mut self);
}

/// Wrapper for the [`reqwest::Client`] struct that contains the token
/// and the actual url that is used to make the request.
/// It is used to make requests to the API.
pub(crate) async fn get(request: &impl ToRequest) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    match client
        .get(format!(
            "https://www.anapioficeandfire.com/api/{}",
            request.to_request()
        ))
        .send()
        .await
    {
        Ok(response) => {
            let response = response.error_for_status()?;
            response.text().await
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requester::request::BookRequest;

    #[tokio::test]
    async fn test_get() {
        let request = BookRequest::default();
        let response = get(&request).await;
        assert!(response.is_ok());
        assert!(!response.unwrap().is_empty());
    }
}
