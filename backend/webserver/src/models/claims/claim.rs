use chrono::{
    Duration::minutes,
    Utc,
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,           // Subject (user id, etc.)
    exp: usize,            // Expiration time (as a timestamp)
}

impl Claims {
    pub fn new(sub: Uuid, exp: usize) -> Self {

        Self{
            sub: sub.to_string(),
            exp,
        }
    }
}