use mongodb::bson::{oid::ObjectId};
use serde::{
    Serialize,
    Deserialize,
};

use sqlx::FromRow;
use chrono::NaiveDateTime;
#[derive(Deserialize, FromRow , Serialize, Debug)]
pub struct Plan{

    pub id          : i64,
    pub public_id   : String,
    pub code        : String,
    pub name        : String,
    pub description : String,
    pub created_at  : NaiveDateTime,
    pub updated_at  : NaiveDateTime,
}