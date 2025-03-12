use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{
    Serialize,
    Deserialize,
};


pub struct Plan{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id          : Option<ObjectId>,
    pub public_id   : String,
    pub name        : String,
    pub description : String,
    pub created_at  : DateTime,
    pub updated_at  : DateTime,
}