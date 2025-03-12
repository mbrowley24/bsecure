use actix_web::{Scope};



pub fn configure_routes(cfg: &mut web::ServiceConfig) -> Scope {
    cfg.service()

}