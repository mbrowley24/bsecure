
use bcrypt::{hash, DEFAULT_COST};
use chrono::{
    Local,
};
use crate::constants::tier_constants::{
    FREE, PAID, BUSINESS, ENTERPRISE
};
use crate::models::user::{
    register::Register,
    user_plan_tier::UserPlanTier
};

use sqlx::{PgPool, Postgres, Row};

use uuid::Uuid;
use crate::models::user::user::User;

//creates a new user generates Uuid and return it upon a successful save
pub async fn create_new_user(db_pool : &PgPool, new_user : Register) -> Result<Uuid, sqlx::Error> {

    //Generate Uuid
    let public_id : Uuid = generate_uuid(db_pool).await;

    //Generate hash password using bcrypt and plain text password
    let password_hash : String = hash(new_user.password, DEFAULT_COST).unwrap();
    let current_time = Local::now().naive_local();

    sqlx::query(
        "INSERT INTO besecure_proj.users (public_id, username ,firstname, lastname, password,
            phone, email,email_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
    .bind(public_id)
    .bind(&new_user.username)
    .bind(&new_user.firstname)
    .bind(&new_user.lastname)
    .bind(password_hash)
    .bind(&new_user.phone_number)
    .bind(&new_user.email)
    .bind(&new_user.email_verified)
    .bind(current_time)
    .bind(current_time)
    .execute(db_pool)
    .await?;

    Ok(public_id)
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

