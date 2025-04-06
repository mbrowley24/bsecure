use actix_web::web;
use crate::routes::user_routes::{edit_user, login, logout, register_user};
use crate::routes::pcap_routes::upload_pcap;
pub fn configure_routes(cfg: &mut web::ServiceConfig){

    cfg.service(
        web::scope("/api/v1")
            .service(upload_pcap)
            .service(login)
            .service(logout)
            .service(register_user)
            .service(edit_user)
    );

}