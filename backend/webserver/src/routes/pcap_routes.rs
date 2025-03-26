use actix_web::{
    get, post, put,
    web::{self, Path},
    HttpResponse, Responder, Scope,
};
use actix_multipart::Multipart;
use crate::services::pcap_services;
use uuid::Uuid;



/// upload pcap takes in a file from the user to analyze.
#[post("/upload")]
async fn upload_pcap(mut payload: Multipart) -> impl Responder {

    //Todo need to add permission for subscription tiers.
    //Todo identify the user plan here before processing the file
    //free tier customers will get 5 uploads total with limited analysis


    let save = pcap_services::save_pcap_record(payload).await?;


    println!("✅ Uploaded: {}", file_path);
    //         HttpResponse::Ok().body(format!("Received '{}', file saved to '{}'", title, file_path))
}

#[get("/document/{uuid}")]
async fn get_captures(user_id: Path<Uuid>) -> impl Responder {
    // get pcaps for the logged-in user
    HttpResponse::Ok().body(format!("GET captures for user_id: {}", user_id))
}

#[put("/document/{uuid}")]
async fn update_capture(user_id: Path<Uuid>) -> impl Responder {
    // update or re-upload pcap
    HttpResponse::Ok().body(format!("PUT capture for user_id: {}", user_id))
}

#[get("/{uuid}/check")]
async fn get_pcap_check(user_id: Path<Uuid>) -> impl Responder {
    // check status of capture
    HttpResponse::Ok().body(format!("Check capture for user_id: {}", user_id))
}

pub fn configure() -> Scope {
    web::scope("/pcap")
        .service(upload_pcap)

}
