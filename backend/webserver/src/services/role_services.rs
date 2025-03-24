
use chrono::{Local, NaiveDateTime};
use crate::models::role;
use sqlx::{PgPool, Postgres, Row};
use uuid::Uuid;
use crate::models::role::model::Role;
use crate::services::common_services::{generate_uuid, exists_name};

pub async fn create_new_role(db_pool : &PgPool, name : &str) -> Result<Uuid, sqlx::Error> {

    let public_id : Uuid = Uuid::new_v4();
    let current_time : NaiveDateTime  = Local::now().naive_local();
    sqlx::query(
        "INSERT INTO besecure_proj.roles (public_id, name, created_at, updated_at) VALUES
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


pub async fn create_roles(db_pool : &PgPool) -> Result<(), sqlx::Error> {

    let roles : Vec<String> = vec![String::from("customer"),
                                   String::from("admin"),
                                   String::from("developer"),
    ];

    for role in roles {

        //checks if role exists
        let role_exists : bool = exists_name(db_pool, &role, "besecure_proj.roles").await;

        // skip if role name exists in db
        if role_exists {
            println!("Role {} already exists", role);
            continue;
        }

        create_new_role(db_pool, &role).await?;

        println!("Role {} created", role);
    }

    Ok(())
}







pub async fn find_by_uuid(db_pool: &PgPool, role_id : &Uuid) -> Result<Role, sqlx::Error> {

    let role: Role = sqlx::query_as::<Postgres, Role>("SELECT id, public_id, name, created_at,
       updated_at FROM roles WHERE public_id = $1")
    .bind(role_id)
    .fetch_one(db_pool)
    .await?;

    Ok(role)
}



