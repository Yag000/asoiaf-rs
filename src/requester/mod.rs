use self::pagination::Pagination;

pub mod pagination;

/// Trait that is used to convert a struct to a url.
/// It is used to make requests to the API.
pub(crate) trait ToUrl {
    fn to_url(&self) -> String;
}

/// Struct that is used to make book requests to the API.
pub(crate) struct BookRequest {
    pagination: Option<Pagination>,
}

impl ToUrl for BookRequest {
    fn to_url(&self) -> String {
        let mut url = String::from("books");
        if let Some(pagination) = &self.pagination {
            url.push_str(&pagination.to_url());
        }
        url
    }
}

/// Wrapper for the [`reqwest::Client`] struct that contains the token
/// and the actual url that is used to make the request.
/// It is used to make requests to the API.
pub async fn get(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    match client
        .get(format!("https://www.anapioficeandfire.com/api/{}", url))
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

    #[tokio::test]
    async fn test_get() {
        let response = get("books").await;
        assert!(response.is_ok());
        assert!(!response.unwrap().is_empty());
    }
}
