use reqwest::Client;
use sqlx::{Pool, Postgres};
pub struct State {
    pub quick_wit : Client,
    pub pg_client  : Pool<Postgres>,

}

impl State {
    pub fn new(quick_wit: Client, pg_client: Pool<Postgres>) -> Self {
        Self {quick_wit, pg_client}
    }
}