use chrono::{Local, NaiveDateTime};

use serde::{
    Serialize,
    Deserialize,
};

use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Default, Clone, FromRow, Serialize, Deserialize)]
pub struct Vertical {
    id         : int64,
    public_id  : Uuid,
    name       : String,
    created_at : Option<NaiveDateTime>,
    updated_at : Option<NaiveDateTime>,
}