
use serde::{
    Serialize,
    Deserialize,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id                   : Option<i32>,
    pub public_id            : Option<Uuid>,
    pub username             : String,
    pub email                : String,
    pub email_confirmed      : String,
    pub email_verified       : bool,
    pub firstname            : String,
    pub lastname             : String,
    pub password             : String,
    pub password_confirm     : String,
    pub phone_number         : String,
    pub phone_number_confirm : String,



}