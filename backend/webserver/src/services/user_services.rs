use actix_web::web::Data;

use bcrypt::{hash, DEFAULT_COST};
use crate::models::user::{
    register::Register
};

use sqlx::{PgPool, FromRow};
use uuid::Uuid;






pub async fn create_new_user(db_pool : &PgPool, new_user : Register) -> Result<(), sqlx::Error> {

    //Generate Uuid
    let public_id : Uuid = generate_uuid(db_pool).await;
    println!("{}", public_id);
    //Generate hash password using bcrypt and plain text password
    let password_hash : String = hash(new_user.password, DEFAULT_COST).unwrap();

    sqlx::query(
        "INSERT INTO besecure_proj.users (public_id, first_name, last_name, password, phone, email,\
        email_verified) VALUES ($1, $2, $3, $4, $5, $6, $7)")
    .bind(public_id)
    .bind(&new_user.firstname)
    .bind(&new_user.lastname)
    .bind(password_hash)
    .bind(&new_user.phone_number)
    .bind(&new_user.email)
    .bind(&new_user.email_verified)
    .execute(db_pool)
    .await?;

    Ok(())
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

//uuid_exists check if the uuid exists in database
async fn uuid_exists(pool : &PgPool, public_id : &Uuid) -> bool {

    sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM users WHERE public_id = $1)"
    )
    .bind(public_id.to_string())
    .fetch_one(pool)
    .await
    .unwrap_or(false)
}

