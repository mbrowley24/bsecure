use mongodb::{Client, Database};
use sqlx::{Pool, Postgres, Error};
use std::env;
use dotenv::dotenv;



pub async fn connect_to_mongodb() -> Result<Client, mongodb::error::Error> {
    dotenv().ok();

    let uri = env::var("MONGO_URI").expect("MONGO_URI not found.");

    let client = Client::with_uri_str(&uri)
        .await?;

    Ok(client)
}

pub async fn connect_to_postgres() -> Result<Pool<Postgres>, Error>{
    dotenv().ok();

    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let db_pool = Pool::<Postgres>::connect(&db_uri)
        .await?;

    Ok(db_pool)
}
