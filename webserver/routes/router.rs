use actix_web::{Scope, web};



pub fn configure_routes() -> Scope {
    web::scope("/api")
        
}