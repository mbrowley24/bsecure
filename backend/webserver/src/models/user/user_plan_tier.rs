use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct UserPlanTier {
    pub id        : i64,
    pub public_id : Uuid,
    pub plan_id   : i64,
    pub plan_name : String,

}