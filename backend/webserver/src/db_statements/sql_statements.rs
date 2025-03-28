use sqlx::PgPool;
use uuid::Uuid;

pub fn insert(table: String,
              insert_fields: String,
              insert_values: String,
              return_fields: String

) -> String {

    format!("INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
            table,
            insert_fields,
            insert_values,
            return_fields
        )
}


pub async fn exists_uuid(db_pool: &PgPool, public_id : Uuid, table_name: &str) -> bool {

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
