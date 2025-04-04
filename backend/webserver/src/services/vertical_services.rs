use crate::constants::table_names::{
    VERTICAL_TABLE
};
use chrono::{Local, NaiveDateTime};
use sqlx::PgPool;
use uuid::Uuid;
use crate::db_statements::sql_statements::{
    exists_name,
    insert,

};
use crate::services::common_services::generate_uuid;

pub async fn create_verticals(pg_pool: &PgPool) -> Result<(), sqlx::Error> {

    let verticals: Vec<String> = vec![
        "Healthcare & Life Sciences".to_string(),
        "Financial Services".to_string(),
        "Retail & E-commerce".to_string(),
        "Manufacturing".to_string(),
        "Telecommunications".to_string(),
        "Energy & Utilities".to_string(),
        "Government & Public Sector".to_string(),
        "Education".to_string(),
        "Transportation & Logistics".to_string(),
        "Construction & Real Estate".to_string(),
        "Technology & Software".to_string(),
        "Media & Entertainment".to_string(),
        "Hospitality & Travel".to_string(),
        "Agriculture & Food Services".to_string(),
        "Legal & Compliance".to_string(),
    ];

    for vertical in verticals {

        //check if name exists in database
        let exists = exists_name(pg_pool,
                                       vertical.as_str(),
                                       VERTICAL_TABLE
        ).await;

        //if the name exists skip iteration
        if exists {
            continue;
        }

        //create current time and covert to NaiveDateTime
        let current_time : NaiveDateTime = Local::now().naive_local();

        //Generate public_id from uuid
        let public_id : Uuid = generate_uuid(pg_pool, VERTICAL_TABLE).await.map_err(|_| {
            sqlx::Error::WorkerCrashed
        })?;

        let fields : String = "public_id, name, updated_at, created_at".to_string();

        let values :String = "$1, $2, $3, $4".to_string();

        let insert_statement : String = insert(
                                            VERTICAL_TABLE.to_string(),
                                            fields,
                                            values,
                                            "id".to_string()
        );


        sqlx::query(insert_statement.as_str())
            .bind(public_id)
            .bind(vertical)
            .bind(current_time)
            .bind(current_time)
            .execute(pg_pool)
            .await?;
    }

    Ok(())
}