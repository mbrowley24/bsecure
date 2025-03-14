use actix_web::web::Data;
use models::p_capture;
use mongodb::{Collection, Database};


pub fn get_pcap_collection(db: Data<Arc<Database>>) -> Collection<p_capture::Capture> {

}