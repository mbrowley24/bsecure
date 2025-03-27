use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow , Serialize, Deserialize)]
pub struct PacketFile {
    pub public_id  : Option<Uuid>,
    pub name       : Option<String>,
    pub path       : Option<String>,
    pub owner_id   : Option<i64>,
    pub created_at : Option<NaiveDateTime>,
    pub updated_at : Option<NaiveDateTime>,
}

