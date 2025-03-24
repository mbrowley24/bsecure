use mongodb::{
    Client as mongoClient,
    Database
};
use reqwest::Client;
use sqlx::{Pool, Postgres};
pub struct State {
    pub http_client : Client,
    pub mongo_database : Database,
    pub pg_client   : Pool<Postgres>,

}

impl State {
    pub fn new(http_client: Client, mongo_database: Database, pg_client: Pool<Postgres>) -> Self {
        Self {http_client, mongo_database, pg_client}
    }
}