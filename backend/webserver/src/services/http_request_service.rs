use actix_web::cookie::Cookie;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;


struct Claims {
    sub  : String,
    exp  : usize,
    role : String,
}

async fn set_cookie(public_id: Uuid) -> Cookie {

    Cookie::build("", public_id.to_string())
        .path("/api/v1")
        .http_only(true)
        .finish()
}


pub async fn uuid_test() -> Uuid{

    Uuid::parse_str("").unwrap()
}

