use actix_web::{
    get, post, put,
    web::{self, Path},
    HttpResponse, Responder, Scope,
};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

#[post("/upload")]
async fn upload_pcap(mut payload: Multipart) -> impl Responder {

    let mut title: Option<String> = None;
    let mut file_path: Option<String> = None;

    while let Some(Ok(mut field)) = payload.next().await {
        let name = field.name().to_string();
        println!("name: {}", name);
        if name == "title" {

            let mut data = Vec::new();

            while let Some(Ok(chunk)) = field.next().await {

                data.extend_from_slice(&chunk);
            }

            title = Some(String::from_utf8(data).unwrap());

        } else if name == "file" {
            let file_name = field
                .content_disposition()
                .get_filename()
                .map(|f| sanitize_filename::sanitize(f))
                .unwrap_or_else(|| "upload_pcap.pcap".into());

            println!("file_name: {}", file_name);

            let path = format!("./uploads/{}", file_name);

            let mut f = File::create(&path).unwrap();

            while let Some(Ok(chunk)) = field.next().await {
                f.write_all(&chunk).unwrap();
            }

            file_path = Some(path);
        }
    }


    match (title, file_path) {

        (Some(title), Some(file_path)) => {

            println!("✅ Uploaded: {}", file_path);
            HttpResponse::Ok().body(format!("Received '{}', file saved to '{}'", title, file_path))

        }

        _ => {

            HttpResponse::BadRequest().body("Please specify both title and file")
        }
    }
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
