use mongodb::{Client as MongoClient};
use sqlx::{Pool, Postgres};
pub struct State {
    mongo_client : MongoClient,
    pg_client     : Pool<Postgres>,

}

impl State {
    pub fn new(mongo_client: MongoClient, pg_client: Pool<Postgres>) -> Self {
        Self {mongo_client, pg_client}
    }
}