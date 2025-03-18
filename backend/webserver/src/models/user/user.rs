use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDate;
use uuid::Uuid;
use serde::{
    Serialize,
    Deserialize,
};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User{

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id              : Option<u64>,
    pub public_id       : Uuid,
    pub username        : String,
    pub email           : String,
    pub firstname       : String,
    pub lastname        : String,
    pub hashed_password : String,
    pub created_at      : NaiveDate,
    pub updated_at      : NaiveDate,
}