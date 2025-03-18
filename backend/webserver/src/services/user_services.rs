use actix_web::web::Data;
use bcrypt::{hash, verify, DEFAULT_COST};
use models::user::{User, Register};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

pub fn get_user_collection(db: Data<Arc<Database>> ) -> Collection<User::User> {

}


pub fn create_new_user(pool : &PgPool, new_user :Register) -> Result<(), sqlx::Error> {

    let uuid = generate_uuid(pool);

    let user = sqlx::query_as::<_, User>();

    let password_hash = get_password_hash(new_user.password.clone())?;

    sqlx::query!(
        "INSERT INTO users (public_id, first_name, last_name, password, phone, email,\
         email_verified VALUE ($1, $2, $3, $4, $5, $6, $7)"
        uuid,
        new_user.first_name,
        new_user.last_name,
        password_hash,
        new_user.phone,
        new_user.email,
        false,
    )
    .execute(pool)
    .await?;

    Ok(())
}


//generate hashed password
fn generate_hashed_password(pool: &PgPool, plain_password : String) -> String {

    let hashed_password = hash(plain_password, DEFAULT_COST).expect("hashing password failed");

    return hashed_password;
}

//generate uuid for users
async fn generate_uuid(pool: &PgPool) -> Uuid {

    loop {
        let uuid = Uuid::new_v4();

        if !uuid_exists(pool, uuid).await {
            return uuid;
        }

    }
}

//uuid_exists check if the uuid exists in database
async fn uuid_exists(pool : &PgPoo1, uuid : Uuid) -> Result<bool, sqlx::Error> {

    let result = sqlx::query(
        "SELECT EXISTS(SELECT 1 FROM users WHERE uuid = $1) AS exists",
        uuid
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(record) => Ok(record.exists().unwrap_or(false)),
        Err(_) => Ok(false),
    }
}

