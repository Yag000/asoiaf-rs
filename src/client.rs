use crate::{error::Error, item::Book, requester::get};

pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_books(&self) -> Result<Vec<Book>, Error> {
        let answer = get("books").await?;
        let answer = serde_json::from_str::<Vec<Book>>(&answer)?;
        Ok(answer)
    }
}
