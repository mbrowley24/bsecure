use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Role {

    pub id         : i64,
    pub public_id  : Uuid,
    pub name       : String,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime,
}