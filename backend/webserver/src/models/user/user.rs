use bcrypt::{hash, DEFAULT_COST};
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{
    Serialize,
    Deserialize,
};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User{

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id              : Option<ObjectId>,
    pub public_id       : String,
    pub username        : String,
    pub email           : String,
    pub firstname       : String,
    pub lastname        : String,
    pub hashed_password : String,
    pub created_at      : DateTime,
    pub updated_at      : DateTime,
}