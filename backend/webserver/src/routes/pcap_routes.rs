use std::collections::HashMap;
use actix_web::{get, post, put, web::{self, Path}, HttpRequest, HttpResponse, Responder, Scope};
use actix_multipart::Multipart;
use crate::app_state::state::DatabasePool;
use crate::constants::table_names::PCAP_FILES_TABLE;
use crate::services::{
    common_services::generate_uuid,
    http_request_service,
    pcap_services::{
        create_pcap_record,
        pcap_tier_size,
        remove_pcap_file,
        save_pcap_record,
    },

    user_services::get_user_tier,
};
use uuid::Uuid;
use crate::errors::pcap_upload_errors::PcapError;
use std::sync::Arc;
use chrono::Local;

/// upload pcap takes in a file from the user to analyze.
#[post("/upload")]
async fn upload_pcap(req: HttpRequest,
                     db_pool: web::Data<Arc<DatabasePool>>,
                     mut payload: Multipart) -> impl Responder {

    //method map to pass variables around the function
    let mut method_map : HashMap<String, String> = HashMap::new();

    //postgres connection pool
    let pg_pool = &db_pool.pg_pool;

    //Todo place holder for user uuid gathered from the cookie
    let public_id : Uuid = http_request_service::uuid_test().await;
    let id : i64 = 0;

    //Todo need to add permission for subscription tiers. ***still testing
    let user_tier_result : Result<String, PcapError> = get_user_tier(pg_pool, public_id)
        .await
        .map_err(|_| PcapError::Other);

    //assign user tier to user_tier or throw forbidden error of error
    match user_tier_result {
        Ok(tier) => method_map.insert("user_tier".to_string(), tier),
        Err(e) => return HttpResponse::Forbidden()
    }

    let max_pcap_file_size : usize = pcap_tier_size(&method_map["user_tier"]);

    //if file is saved and hashmap is returned the operation was successful.
    //else error is sent back to frontend with custom message
    match save_pcap_record(payload, max_pcap_file_size).await{

        Ok(map) => {

            method_map.insert("file_name".to_string(), map["file_name"].to_string());
            method_map.insert("file_path".to_string(), map["file_path"].to_string());
        }

        Err(err) => return HttpResponse::BadRequest().json(err)
    }

    //save record of new file to database with file name, filepath and owner_id

    let current_time = Local::now().naive_local();

    let new_uuid : Uuid = generate_uuid(pg_pool, &PCAP_FILES_TABLE);

    match create_pcap_record(pg_pool,
                             &method_map["file_name"],
                            &method_map["file_path"],
                             id,
                             new_uuid,
                             current_time,
                            ).await {

        //recorded created successfully
        Ok(pcap_record) => {
            let message: String = format!("Successfully created pcap record: {}", Ok(pcap_record.name));
            HttpResponse::Ok().json(message)
        }

        //error recording record start the process of removing the pcap file and returning an error
        Err(err) => {
            match remove_pcap_file(&method_map["file_path"]){

                Ok(file_path) =>{
                    let message: String = format!("File {} removed", method_map["file_name"]);
                    HttpResponse::InternalServerError().json(message)
                }

                _=> HttpResponse::InternalServerError().body({})
            }
        }
    }


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
