use std::collections::HashMap;
use actix_multipart::{Field, Multipart};
use chrono::{Local, NaiveDateTime};
use futures_util::{StreamExt, TryFutureExt};

use crate::constants::{
    pcap_constants::PcapSizes,
    table_names::PCAP_FILES_TABLE,
    tier_constants::{
        PAID, FREE, BUSINESS, ENTERPRISE
    }
};

use crate::db_statements::sql_statements::{
    insert
};
use crate::errors::pcap_upload_errors::PcapError;
use crate::services::{
    common_services::generate_uuid,
    user_services::get_user_tier
};
use crate::models::packet_data::{
    packet_file::PacketFile,
};
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
        Err(_) => Err(PcapError::FileNotSavedError)
    }

}



///creates method that save a record in the database about the pcap file.
/// Throws an error if error occurs during inserting data into the database
pub async  fn create_pcap_record(db_pool: &PgPool,
                                 name         : &str,
                                 path         : &str,
                                 owner_id     : i64,
                                 public_id    : Uuid,
                                 current_time : NaiveDateTime
) -> Result<PacketFile, PcapError> {

    let query_statement : String = insert(
        PCAP_FILES_TABLE,
        "public_id, name, path, owner_id, created_at, updated_at".to_string(),
        "$1, $2, $3, $4, $5, $6)".to_string(),
        "id, name ,path".to_string()
    );

    let packet_file : PacketFile = sqlx::query_as::<_, PacketFile>(&query_statement)
                    .bind(public_id)
                    .bind(name)
                    .bind(path)
                    .bind(owner_id)
                    .bind(current_time)
                    .bind(current_time)
                    .fetch_one(db_pool)
                    .await.map_err(|_| PcapError::FileNotSavedError)?;

    Ok(packet_file)
}


/// This function is used to determine the size of the pcap allowed based on subscription type
pub async fn pcap_tier_size(user_tier_string: &str) -> usize {

        match user_tier_string {
            PAID       => PcapSizes::max_bytes(&PcapSizes::Free),
            BUSINESS   => PcapSizes::max_bytes(&PcapSizes::Business),
            ENTERPRISE => PcapSizes::max_bytes(&PcapSizes::Enterprise),
            _          => PcapSizes::max_bytes(&PcapSizes::Free),
        }

}



///remove pcap_file removes the file if there is an error during processing
pub fn remove_pcap_file(filepath: &str) -> Result<&str, PcapError> {

    match remove_file(filepath).map_err(|_| PcapError::FileNotSavedError){
        Ok(_) => Ok(filepath),
        Err(_) => Err(PcapError::FailedToRemoveFile(filepath.to_string()))
    }

}

pub async fn save_pcap_record(mut payload :Multipart,
                              max_pcap_size: usize
) -> Result<HashMap<String, String>, PcapError>{

    //return file data: file path and file name
    let mut file_data : HashMap<String, String> = HashMap::new();



    while let Some(Ok(mut field)) = payload.next().await {

        if let Some(filename) = field.content_disposition().get_filename() {

            file_data.insert(String::from("filename"), safe_filename(filename));

            //validate file type
            validate_file_type(&Ok(file_data.get("filename")))?;

            file_data.insert(
                String::from("filepath"),
                format!("./files/{}.pcap", safe_filename)
            );

            //crate new pcap file if error return error
            let mut f : File = create_file(&Ok(file_data.get("filepath")))?;



            //write the bytes to pcap and check for error
            write_to_pcap(&mut f, &mut field, &filename, max_pcap_size).await?;


        };

    }

    Ok(file_data)
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

///write bytes to pcap file, checks for file size based on customer tier
/// return PcapError if there is an issue creating the file or if the file becomes too large
async fn write_to_pcap(file          : &mut File,
                       field         : &mut Field,
                       filepath      : &str,
                       max_file_size : usize
) -> Result<(), PcapError>{

    //container to measure for file size
    let mut file_size : usize = 0;

    while let Some(Ok(chunk)) = field.next().await {

        //Add current bytes to file size
        file_size += chunk.len();

        //check max file size is grater than max file size
        if file_size > max_file_size{

            //remove filepath when file is larger than file size
            remove_pcap_file(filepath)?;

            return Err(PcapError::FileTooLarge)
        }

        //write bits to the file
        let result : Result<(), PcapError> = file.write_all(&chunk).await;

        //check for errors
        match result {

            Ok(_) => {},

            Err(_) =>{
                return  Err(PcapError::IOError)
            }
        }
    }

    Ok(())

}





