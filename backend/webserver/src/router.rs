use actix_web::{web, Scope};
use crate::routes;


pub fn configure() -> Scope{

    web::scope("/api/v1")
        // .service(routes::user_routes::configure())
        .service(routes::pcap_routes::configure())

}