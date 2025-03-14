mod user{
    use User;
}
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{
    Serialize,
    Deserialize,
};




#[derive(Debug, Serialize, Deserialize)]
pub struct Capture {

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id           : Option<ObjectId>,
    pub public_id    : String,
    pub title        : String,
    pub path         : String,
    pub description  : String,
    pub owner        : User,
    pub created_at   : DateTime,
    pub updated_at   : DateTime,
}