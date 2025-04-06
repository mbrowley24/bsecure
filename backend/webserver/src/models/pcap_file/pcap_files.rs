use chrono::NaiveDateTime;
use crate::services::common_services::{
    generate_uuid,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Default ,Deserialize, FromRow , Serialize)]
pub struct PcapFiles {

    pub id         : Option<i64>,
    pub public_id  : Option<Uuid>,
    pub file_name  : Option<String>,
    pub file_path  : Option<String>,
    pub owner_id   : Option<i64>,
    pub updated_at : Option<NaiveDateTime>,
    pub created_at : Option<NaiveDateTime>,

}

impl PcapFiles {

    pub fn new() -> PcapFiles {
        Self{
            ..Default::default()
        }
    }
}