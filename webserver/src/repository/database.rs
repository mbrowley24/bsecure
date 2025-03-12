use mongodb::{Client, Database};
use std::env;
use dotenv::dotenv;


pub async fn connect_to_mongodb() -> Result<Database, mongodb::error::Error> {
    dotenv().ok();

    let uri = env::var("MONGO_URI").expect("MONGO_URI not found.");
    let db_name = env::var("MONGO_DB").expect("MONGO_DB_NAME not found.");

    let client = Client::with_uri_str(&uri)
        .await?;

    Ok(client.database(&db_name))
}