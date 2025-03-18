
use crate::json_schemas;
use crate::models::{
    user::{
        register::Register,
        user::User
    }
};

use actix_web::{
    get,
    HttpResponse,
    post,
    put,
    Responder,
    Scope,
    web,
};


#[post("/login")]
async fn login() -> impl Responder {

    let data = json_schemas::user::Test::new(200, String::from("this is a test1"));

    HttpResponse::Ok().json(data)

}

#[post("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Logout")
}


#[post("/register")]
async fn register(new_user: web::Json<Register>) -> impl Responder {


    HttpResponse::Ok().body("")
}

pub fn configure() -> Scope {
    web::scope("/api/v1")
        .service(login)
        .service(logout)
}