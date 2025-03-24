use dotenv::dotenv;
use mongodb::{Client as mongoClient, options::ClientOptions, Database};
use reqwest::{Client, Error as R_Error};
use std::env;
use sqlx::{Pool, Postgres, Error};



pub async fn connect_to_mongodb() -> mongodb::error::Result<Database>{
    dotenv().ok();

    let uri = env::var("MONGO_URI").expect("mongo database not found");

    let mut options = ClientOptions::parse(uri).await?;

    let client = mongodb::Client::with_options(options)?;

    let database = env::var("MONGO_DB").expect("mongo database not found");

    Ok(client.database(database.as_str()))
}



pub async fn connect_to_postgres() -> Result<Pool<Postgres>, sqlx::Error>{
    dotenv().ok();

    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let db_pool = Pool::<Postgres>::connect(&db_uri)
        .await?;

    Ok(db_pool)
}


pub async fn http_client() -> Result<Client, R_Error> {

    let client = Client::builder()
        .pool_max_idle_per_host(10)
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .expect("Client building request failed");

    Ok(client)
}