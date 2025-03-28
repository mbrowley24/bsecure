use std::fmt::Debug;
use actix_web::{
    get,
    HttpResponse,
    post,
    put,
    Responder,
    Scope,
    web,
};
use crate::app_state::state::DatabasePool;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Local;
use crate::constants::table_names::{
    USERS_TABLE
};
use crate::json_schemas;
use crate::models::{
    user::{
        dto::DTO,
        user::User
    }
};

use crate::services::{
    user_services::{
        create_new_user,
        get_user_by_pub_id
    },
    common_services::{
        generate_uuid,
    }
};
use std::sync::Arc;

use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;



#[get("edit/{id}")]
async fn edit_user(app_data : web::Data<Arc<DatabasePool>>,
                   id : web::Path<Uuid>) -> impl Responder {

    let pg_pool: &Pool<Postgres> = &app_data.pg_pool;

    let result = get_user_by_pub_id(pg_pool, id.into_inner()).await;

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
async fn register_user(app_data : web::Data<Arc<DatabasePool>>
                       ,mut reg_user: web::Json<DTO>
) -> impl Responder {

    let pg_pool: &Pool<Postgres> = &app_data.pg_pool;
    let new_user : User;

    //generate uuid generates uuid and validates the valid is unique
    let uuid : Uuid = match generate_uuid(pg_pool, USERS_TABLE).await{
        Ok(uuid ) => uuid ,

        Err(_) => return HttpResponse::InternalServerError().json({})
    };

    //assign unique uuid
    reg_user.set_public_id(uuid);


    //Generate hash password using bcrypt and plain text password
    let hashed_password = match hash(&reg_user.password, DEFAULT_COST){

        Ok(hashed) => hashed,
        Err(_) => return HttpResponse::InternalServerError().json({})
    };

    reg_user.set_hashed_password(hashed_password);

    reg_user.set_current_time(Local::now().naive_local());

    let new_user =  match User::register_to_new(reg_user.into_inner()){

        Some(new_user) => new_user,
        None => return HttpResponse::InternalServerError().json({})
    };

    let query_string = match new_user.insert_new_user_query(){
        Ok(query_string) => query_string,
        Err(_) => return HttpResponse::InternalServerError().json({})
    };

    let result = create_new_user(pg_pool, query_string)
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
    web::scope("/api/v1")
        .service(login)
        .service(logout)
        .service(register_user)
        .service(edit_user)
}