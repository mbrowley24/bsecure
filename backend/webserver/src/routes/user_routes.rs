use actix_web::{
    get,
    HttpResponse,
    post,
    put,
    Responder,
    Scope,
    web,
};
use crate::app_state;
use crate::json_schemas;
use crate::models::{
    user::{
        register::Register,
        user::User
    }
};

use crate::services::user_services;
use std::sync::Arc;
use sqlx::{PgPool};



#[post("/login")]
async fn login() -> impl Responder {

    let data = json_schemas::user::Test::new(200, String::from("this is a test1"));

    HttpResponse::Ok().json(data)

}

#[post("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Logout")
}


#[post("register")]
async fn register(app_data : web::Data<Arc<app_state::state::State>> ,new_user: web::Json<Register>) -> impl Responder {

    println!("{:?}", new_user);

    let db_client = &app_data.pg_client;

    let result = user_services::create_new_user(db_client, new_user.into_inner())
        .await;
    
    match result  {

        Ok(user) => {
            HttpResponse::Ok().body("success")
        }

        Err(error) => {
            println!("{:?}", error);
            HttpResponse::InternalServerError().body("Something went wrong")
        }
    }

}

pub fn configure() -> Scope {
    web::scope("")
        .service(login)
        .service(logout)
        .service(register)
}