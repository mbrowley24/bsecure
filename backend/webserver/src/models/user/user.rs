use bcrypt::{hash, DEFAULT_COST};
use chrono::{Local, NaiveDateTime};
use uuid::Uuid;
use serde::{
    Serialize,
    Deserialize,
};
use sqlx::FromRow;

#[derive(Debug, Default, Clone, FromRow, Serialize, Deserialize)]
pub struct User{

    //#[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id              : i64,
    // #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id       : Uuid,
    pub username        : String,
    pub email           : Option<String>,
    pub firstname       : String,
    pub lastname        : String,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password        : Option<String>,
    pub created_at      : NaiveDateTime,
    pub updated_at      : NaiveDateTime,
}


impl User {

}