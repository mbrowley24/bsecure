use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Test {
    pub status: u16,
    pub message: String,
}

impl Test {
    pub fn new(status: u16, message: String) -> Self {

        Self { status, message }
    }
}