use actix_web::{
    get,
    HttpResponse,
    post,
    put,
    Responder,
    Scope,
    web,
};
use uuid::Uuid;
use crate::json_schemas;


#[get("/pcap/{uuid}")]
async fn get_captures(user_id: web::Path<Uuid>) -> impl Responder{

    //get pcaps for the login user

    HttpResponse::Ok()
}

#[post("/pcap/{uuid}")]
async fn post_capture(user_id : web::Path<Uuid>) -> impl Responder{

    //save an uploaded pcap
    HttpResponse::Ok()
}

#[get("/pcap/{uuid}/check")]
async fn get_pcap_check(user_id: web::Path<Uuid>) -> impl Responder{

    HttpResponse::Ok()
}



