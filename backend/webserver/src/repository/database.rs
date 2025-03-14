use mongodb::{Client, Database};
use std::env;
use dotenv::dotenv;


pub async fn connect_to_mongodb() -> Result<Client, mongodb::error::Error> {
    dotenv().ok();

    let uri = env::var("MONGO_URI").expect("MONGO_URI not found.");

    let client = Client::with_uri_str(&uri)
        .await?;

    Ok(client)
}