use std::io::{BufRead, Read, Write};

use reqwest::{header::HeaderMap, StatusCode};

use crate::error::Error;

use self::pagination::Pagination;

pub mod filter;
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

pub(crate) fn create_cache_dir() {
    std::fs::create_dir_all(CACHE_DIR).unwrap();
}

static CACHE_DIR: &str = ".cache/answers";

fn request_to_file(request: &str) -> String {
    format!("{CACHE_DIR}/{}.json", request.replace("/", "_"))
}

fn is_in_cache(request_name: &str) -> bool {
    let file = request_to_file(&request_name);
    std::path::Path::new(&file).exists()
}

/// The etad of the request is stored in the first line of the file.
/// The rest of the file is the actual response.
fn get_file_requets_etag(request_name: &str) -> Result<String, Error> {
    let file = request_to_file(&request_name);
    let mut file = std::fs::File::open(file).map_err(|e| Error::CacheMiss)?;
    let mut contents = String::new();
    let mut buffer = std::io::BufReader::new(file);
    buffer
        .read_line(&mut contents)
        .map_err(|e| Error::CacheMiss)?;
    Ok(contents)
}

/// Wrapper for the [`reqwest::Client`] struct that contains the token
/// and the actual url that is used to make the request.
/// It is used to make requests to the API.
pub(crate) async fn get(request: &impl ToRequest) -> Result<String, reqwest::Error> {
    let mut headers = HeaderMap::new();

    let request_string = request.to_request();

    if is_in_cache(&request_string) {
        let etag = get_file_requets_etag(&request_string);
        if etag.is_ok() {
            headers.insert(
                reqwest::header::IF_NONE_MATCH,
                reqwest::header::HeaderValue::from_str(&etag.unwrap()).unwrap(),
            );
        }
    }

    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://www.anapioficeandfire.com/api/{}",
            request_string
        ))
        .headers(headers)
        .send()
        .await?;

    let response = response.error_for_status()?;

    if response.status() == StatusCode::NOT_MODIFIED {
        let file_name = request_to_file(&request_string);
        let file = std::fs::File::open(file_name).unwrap();
        let mut contents = String::new();
        // We omit the first line because it is the etag.
        let mut buffer = std::io::BufReader::new(file);
        buffer.read_line(&mut contents).unwrap();
        contents.clear();
        buffer.read_to_string(&mut contents).unwrap();
        Ok(contents)
    } else {
        let text;
        if let Some(etag) = response.headers().get(reqwest::header::ETAG).cloned() {
            let etag = etag.to_str().unwrap();
            let file = request_to_file(&request_string);
            let mut file = std::fs::File::create(file).unwrap();

            text = response.text().await.unwrap();
            file.write_all(etag.as_bytes()).unwrap();
            file.write_all("\n".as_bytes()).unwrap();
            file.write_all(text.as_bytes()).unwrap();
        } else {
            text = response.text().await?;
        }
        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requester::request::BookRequest;

    #[tokio::test]
    async fn test_get() {
        create_cache_dir();
        let request = BookRequest::default();
        let response = get(&request).await;
        assert!(response.is_ok());
        assert!(!response.unwrap().is_empty());
    }
}
