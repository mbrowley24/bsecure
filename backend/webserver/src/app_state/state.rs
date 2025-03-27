use mongodb::{
    Client as mongoClient,
    Database
};
use reqwest::Client;
use sqlx::{Pool, Postgres};
pub struct DatabasePool{
    pub http_client : Client,
    pub mongo_database : Database,
    pub pg_pool   : Pool<Postgres>,

}

impl DatabasePool {
    pub fn new(http_client: Client, mongo_database: Database, pg_pool: Pool<Postgres>) -> Self {
        Self {http_client, mongo_database, pg_pool}
    }
}