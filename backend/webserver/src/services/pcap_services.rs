use actix_multipart::{Field, Multipart};
use chrono::Local;
use futures_util::StreamExt;
use crate::constants::pcap_constants::PcapSizes;
use crate::errors::pcap_upload_errors::PcapError;
use crate::services::common_services::generate_uuid;
use std::collections::HashMap;
use std::fs::{File, remove_file};
use std::io::{Error, ErrorKind, Write};
use sqlx::PgPool;

use uuid::Uuid;





///create_file creates a new file or throws a Pcap error if there
/// is an error during file creation
fn create_file(filename: &str) -> Result<File, PcapError>{

    let new_file =  File::create(&filename);

    match new_file {

        Ok(file) => Ok(file),
        Err(_) => Err(PcapError::FileNoSavedError)
    }

}



///creates method that save a record in the database about the pcap file. Record includes
/// name (filename) , owner_id and the filepath. Throws an error if error occurs during
/// inserting data into the database
pub async  fn create_pcap_record(db_pool: &PgPool,
                                 name: &str,
                                 path: &str,
                                 owner_id : i64
) -> Result<(), sqlx::Error> {

    let public_id : Uuid = generate_uuid(db_pool, "besecure_proj.pcap_files")
        .await.expect("can't generate public ID");

    let current_time = Local::now().naive_local();

    sqlx::query(
        "INSERT INTO besecure_proj.pcap_files (public_id, name, path, owner_id, created_at,
            updated_at) VALUES ($1, $2, $3, $4, $5, $6)"
    )

    .bind(public_id)
    .bind(name)
    .bind(path)
    .bind(owner_id)
    .bind(current_time)
    .bind(current_time)
    .execute(db_pool)
    .await?;


    Ok(())
}

///max_file_size Validates the size of a file. 10Mb is the defaul
fn max_file_size() -> Result<(), PcapError> {


    let mut file_size :usize = 0;

    Ok(())
}


///remove pcap_file removes the file if there is an error during processing
fn remove_pcap_file(filepath: &str) -> Result<(), PcapError> {

    match remove_file(filepath).map_err(|_| PcapError::FileNoSavedError){
        Ok(_) => Ok(()),
        Err(_) => Err(PcapError::FileNoSavedError)
    }

}

pub async fn save_pcap_record(mut payload :Multipart) -> Result<(), PcapError>{
    let mut title: Option<String> = None;
    let mut file_path: Option<String> = None;
    let mut message_map : HashMap<String, String>  = HashMap::new();




    while let Some(Ok(mut field)) = payload.next().await {

        if let Some(filename) = field.content_disposition().get_filename() {

            let safe_filename = safe_filename(filename);

            //validate file type
            validate_file_type(&safe_filename)?;

            let filepath : String = format!("./files/{}.pcap", safe_filename);

            //crate new pcap file if error return error
            let mut f : File = create_file(&filepath)?;


            //write the bytes to pcap and check for error
            write_to_pcap(&mut f, &mut field).await?;




            message_map
                .get_mut("filename")
                .unwrap()
                .push(safe_filename);

            message_map
                .get_mut("paths")
                .unwrap()
                .push(filepath);


        };


    }


    Ok(message_map)



}


///safe_filename removes dangerous character from the provided file name. This function
/// should be the first function ran in the validation process
fn safe_filename(filename : &str) -> String{
    let mut cleaned = sanitize_filename::sanitize(filename);

    if cleaned.len() < 3 {

        cleaned = format!("file_{}", Uuid::new_v4());

    }

    if cleaned.len() > 255 {
        cleaned = cleaned[0..255].to_string();
    }

    cleaned

}


///validate_file_type function validate the file type .pcap or pcapng and assumes
///the filename has been sanitized
fn validate_file_type(filename : &str) -> Result<(), PcapError> {


    if !filename.ends_with(".pcap") && !filename.ends_with(".pcapng") {

        return Err(PcapError::InvalidExtension)
    }


    Ok(())
}

//write bytes to pcap file adn return PcapError
async fn write_to_pcap(file : &mut File, field: &mut Field) -> Result<(), PcapError>{

    while let Some(Ok(chunk)) = field.next().await {

        let result : Result<(), PcapError> = file.write_all(&chunk).await;

        match result {

            Ok(_) => {},

            Err(_) =>{
                return  Err(PcapError::IOError)
            }
        }
    }

    Ok(())

}


// pub fn get_pcap_collection(db_pool : &PgPool, uuid: Uuid) -> Result<(), sqlx::Error> {
//
// }



