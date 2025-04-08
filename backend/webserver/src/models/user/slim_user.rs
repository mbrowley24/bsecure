
use serde::{
    Serialize,
    Deserialize,
};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Default, Clone, FromRow, Serialize, Deserialize)]
pub struct SlimUser{


    pub id              : Option<i64>,
    pub public_id       : Option<Uuid>,
    pub username        : Option<String>,
    pub email           : Option<String>,
    pub firstname       : Option<String>,
    pub lastname        : Option<String>,
    pub email_verified  : bool,
    pub email_key       : Option<String>,
    pub phone           : Option<String>,
}