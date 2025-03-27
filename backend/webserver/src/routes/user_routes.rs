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
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;



#[get("edit/{id}")]
async fn edit_user(app_data : web::Data<Arc<app_state::state::DatabasePool>>,
                   id : web::Path<Uuid>) -> impl Responder {

    let pg_pool: &Pool<Postgres> = &app_data.pg_pool;

    let result = user_services::get_user_by_pub_id(pg_pool, id.into_inner()).await;

    match result {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        }
        Err(e) =>{

            println!("{}", e);
            HttpResponse::BadRequest().body("Invalid request".to_string())
        }
    }


}

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
async fn register_user(app_data : web::Data<Arc<app_state::state::DatabasePool>> ,new_user: web::Json<Register>) -> impl Responder {


    let db_client: &Pool<Postgres> = &app_data.pg_pool;

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
        .service(register_user)
        .service(edit_user)
}