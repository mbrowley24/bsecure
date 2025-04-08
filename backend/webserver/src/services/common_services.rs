use crate::db_statements::sql_statements::{
    exists_name,
    exists_uuid,
};
use rand::{distributions::Alphanumeric, Rng};
use sqlx::PgPool;
use uuid::Uuid;


pub async fn generate_uuid(db_pool: &PgPool, table_name : &str) -> Result<Uuid, sqlx::Error> {


    loop{

        let public_id : Uuid = Uuid::new_v4();

        let exists: bool = exists_uuid(db_pool, public_id, table_name).await;

        if !exists {
            return Ok(public_id)
        }

        println!("generate uuid for {} -> {}", table_name, public_id);
    }
}


pub fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}