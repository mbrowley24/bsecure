
use chrono::{Local, NaiveDateTime};

use sqlx::{PgPool, Postgres, Row};
use uuid::Uuid;
use crate::models::plan::model::Plan;
use crate::services::common_services::{exists_name, generate_uuid};
pub async fn create_new_plan(db_pool : &PgPool, name : &str) -> Result<Uuid, sqlx::Error> {

    let public_id : Uuid = generate_uuid(db_pool, "besecure_proj.plans")
        .await
        .expect("Failed to generate UUID");

    let current_time : NaiveDateTime  = Local::now().naive_local();
    sqlx::query(
        "INSERT INTO besecure_proj.plans (public_id, name, created_at, updated_at) VALUES
            ($1, $2, $3, $4)"
    )
        .bind(public_id)
        .bind(name)
        .bind(current_time)
        .bind(current_time)
        .execute(db_pool)
        .await?;

    Ok(public_id)
}


pub async fn create_plans(db_pool : &PgPool) -> Result<(), sqlx::Error> {

    let plans : Vec<String> = vec![String::from("free"),
                                   String::from("basic"),
                                   String::from("analysis"),
                                   String::from("business"),
                                   String::from("enterprise")
    ];

    for plan in plans {

        //checks if role exists
        let plan_exists : bool = exists_name(db_pool, &plan, "besecure_proj.plans").await;

        // skip if role name exists in db
        if plan_exists {
            println!("Plan {} already exists", plan);
            continue;
        }

        println!("Creating plan {}", plan);

        create_new_plan(db_pool, &plan).await?;

        println!("Plan {} created", plan);
    }

    Ok(())
}





pub async fn find_by_uuid(db_pool: &PgPool, role_id : &Uuid) -> Result<Plan, sqlx::Error> {

    let role: Plan = sqlx::query_as::<Postgres, Plan>("SELECT id, public_id, name, created_at,
       updated_at FROM roles WHERE public_id = $1")
        .bind(role_id)
        .fetch_one(db_pool)
        .await?;

    Ok(role)
}



