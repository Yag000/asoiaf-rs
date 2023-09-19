#[tokio::main]
async fn main() {
    let answer = asoiaf_api::requester::get("books").await;
    println!("{}", answer.unwrap());
}
