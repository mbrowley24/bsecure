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

}

#[post("/logout")]
async fn logout() -> impl Responder {

}


pub fn user_routes() -> Scope {
    web::scope("/api/v1/login")
        .service(login)
}