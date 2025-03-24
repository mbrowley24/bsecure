use sqlx::PgPool;
use uuid::Uuid;

async fn exists_uuid(db_pool: &PgPool, public_id : Uuid, table_name: &str) -> bool {

    sqlx::query_scalar(
        format!("SELECT EXISTS(SELECT 1 FROM {} WHERE public_id = $1)", table_name).as_str()
    )
        .bind(public_id)
        .fetch_one(db_pool)
        .await
        .unwrap_or(false)
}

pub async fn exists_name(db_pool: &PgPool, name : &str, table_name : &str) -> bool {

    sqlx::query_scalar(
        format!("SELECT EXISTS(SELECT 1 FROM {} WHERE name = $1)", table_name).as_str()
    )
        .bind(name)
        .fetch_one(db_pool)
        .await
        .unwrap_or(false)
}

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