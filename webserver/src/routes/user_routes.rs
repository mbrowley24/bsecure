
use crate::json_schemas;

use actix_web::{
    get,
    HttpResponse,
    post,
    put,
    Responder,
    Scope,
    web,
};
use serde_json::json;

#[post("/login")]
pub async fn login() -> impl Responder {

    let data = json_schemas::user::Test::new(200, String::from("this is a test1"));

    HttpResponse::Ok().json(data)

}

#[post("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Logout")
}


pub fn configure() -> Scope {
    web::scope("/api/v1")
        .service(login)
}