
use bcrypt::{hash, DEFAULT_COST};
use chrono::{
    Local,
};
use crate::constants::{
    tier_constants::{
        FREE, PAID, BUSINESS, ENTERPRISE
    },
    table_names::{
        USERS_TABLE
    }
};

use crate::db_statements::sql_statements::{
    insert
};
use crate::models::user::{
    dto::DTO,
    user_plan_tier::UserPlanTier
};

use sqlx::{PgPool, Postgres, Row};

use uuid::Uuid;
use crate::models::user::user::User;

//creates a new user generates Uuid and return it upon a successful save
pub async fn create_new_user(pg_pool  : &PgPool,
                             query    : &str,
                             new_user : User
) -> Result<User, sqlx::Error> {


    let saved_user = sqlx::query_as::<_, User>(query)
        .bind(&new_user.public_id)
        .bind(&new_user.username)
        .bind(&new_user.firstname)
        .bind(&new_user.lastname)
        .bind(&new_user.password)
        .bind(&new_user.phone)
        .bind(&new_user.email)
        .bind(&new_user.email_verified)
        .bind(new_user.created_at)
        .bind(new_user.updated_at)
        .fetch_one(pg_pool)
        .await?;

    Ok(saved_user)
}

pub async fn get_user_by_pub_id(db_pool: &PgPool, public_id: Uuid) -> Result<User, sqlx::Error> {


    let user  = sqlx::query_as::<Postgres, User>(
        "SELECT id, public_id, username, firstname, lastname, email, password, created_at,
              updated_at FROM besecure_proj.users WHERE public_id = $1",

    )
    .bind(public_id)
    .fetch_one(db_pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_username(db_pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {

    let user : User = sqlx::query_as::<Postgres, User>(
        "SELECT id, public_id, username, firstname, lastname, email, password, created_at,
              updated_at FROM besecure_proj.users WHERE username = $1",
    )
    .bind(username)
    .fetch_one(db_pool)
    .await?;

    Ok(user)
}




//generate uuid for users
async fn generate_uuid(pool: &PgPool) -> Uuid {

    loop {
        let uuid = Uuid::new_v4();

        if !uuid_exists(pool, &uuid).await {
            return uuid;
        }

    }
}


pub async fn get_user_tier(db_pool: &PgPool, public_id: Uuid) -> Result<&str, sqlx::Error> {


    let user_tier : UserPlanTier = sqlx::query_as::<Postgres, UserPlanTier>(
        "SELECT users.id        AS id,
                    users.public_id AS public_id,
                    plans.id        AS plan_id,
                    plans.name      AS plan_name,
                    FROM besecure_proj.users WHERE public_id = $1
                    JOIN users on users.plan_id = plans.id",
    )
        .bind(public_id)
        .fetch_one(db_pool)
        .await?;


    match user_tier.plan_name.as_str() {

        PAID       => Ok(PAID),
        BUSINESS   => Ok(BUSINESS),
        ENTERPRISE => Ok(ENTERPRISE),
        _          => Ok(FREE)
    }

}

//uuid_exists check if the uuid exists in database
async fn uuid_exists(pool : &PgPool, public_id : &Uuid) -> bool {

    sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM users WHERE public_id = $1)"
    )
    .bind(public_id)
    .fetch_one(pool)
    .await
    .unwrap_or(false)
}

