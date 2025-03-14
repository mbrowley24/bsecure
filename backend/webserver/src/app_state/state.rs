use mongodb::{Client as MongoClient};
use elasticsearch::{Elasticsearch, };
pub struct State {
    mongo_client : MongoClient,
    es_client    : Elasticsearch,
}

impl State {
    pub fn new(mongo_client: MongoClient, es_client: Elasticsearch) -> Self {
        State { mongo_client, es_client }
    }
}