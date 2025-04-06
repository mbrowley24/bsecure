use actix_web::cookie::Cookie;
use chrono::Utc;
use dotenv::dotenv;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use std::env;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub  : String,
    exp  : usize,
    role : String,
}

async fn set_cookie(jwt: String) -> Cookie<'static> {

    Cookie::build("claims", jwt)
        .path("/api/v1")
        .http_only(true)
        .finish()
}


pub async fn uuid_test() -> Uuid{

    Uuid::parse_str("").unwrap()
}

pub async fn create_jwt(user_id : Uuid) -> jsonwebtoken::errors::Result<String> {

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(4))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims{
        sub: user_id.to_string(),
        exp: expiration,
        role: "".to_string()
    };

    let secret_key = env::var("JWT_SECRET_KEY")
        .expect("JWT_SECRET_KEY must be set");

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
}