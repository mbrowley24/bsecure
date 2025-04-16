use std::collections::HashMap;
use std::fmt::format;
use actix_multipart::{Field, Multipart};
use chrono::{Local, NaiveDateTime};
use futures_util::{StreamExt, TryFutureExt, TryStreamExt};

use crate::constants::{
    pcap_constants::{
        PcapSizes,
        PCAPNG_BYTES
    },
    table_names::PCAP_FILES_TABLE,
    tier_constants::{
        PAID, FREE, BUSINESS, ENTERPRISE
    }
};

use crate::db_statements::sql_statements::{
    insert
};
use crate::errors::pcap_upload_errors::PcapError;

use crate::models::packet_data::{
    packet_file::PacketFile,
    tcp_connection_key::TcpConnectionKey,
};

use pnet_packet::{
    ethernet::{
        EthernetPacket,
    },
    ip::IpNextHeaderProtocols,
    ipv4::{
        Ipv4Packet
    },
    Packet,
    tcp::{
        TcpFlags,
        TcpPacket
    }
};
use std::fs::{File, remove_file};
use std::hash::Hash;
use std::io;
use std::io::Write;
use actix_web::dev::Payload;
use pcap_parser::nom::character::complete::tab;
use pcap_parser::{Block, PcapBlockOwned, PcapCapture, PcapNGCapture, PcapNGReader};
use pnet_packet::ethernet::EtherTypes;
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
        PCAP_FILES_TABLE.to_string(),
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

fn detect_pcap_format(buffer: &[u8]) -> &'static str {

    match buffer.get(..4) {
        Some([0xd4, 0xc3, 0xb2, 0xa1]) => "Legacy PCAP (little-endian)",
        Some([0xa1, 0xb2, 0xc3, 0xd4]) => "Legacy PCAP (big-endian)",
        Some([0x0a, 0x0d, 0x0d, 0x0a]) => "PCAPNG",
        _ => "Unknown format",
    }
}



/// Check for broadcast at Layer 2 and Layer 3
fn is_broadcast(packet_data: &[u8]) -> (bool, bool) {
    // Parse Ethernet frame
    if let Some(ethernet_packet) = EthernetPacket::new(packet_data) {
        // Layer 2 Broadcast MAC Address
        let l2_broadcast = ethernet_packet.get_destination().octets() == [0xFF; 6];

        // Check for IPv4 broadcast if it's an IP packet
        if ethernet_packet.get_ethertype() == EtherTypes::Ipv4 {
            if let Some(ip_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                let dest_ip = ip_packet.get_destination().octets();

                // Layer 3: Limited broadcast (255.255.255.255)
                let l3_broadcast = dest_ip == [255, 255, 255, 255];

                return (l2_broadcast, l3_broadcast);
            }
        }

        return (l2_broadcast, false);
    }

    // If parsing fails
    (false, false)
}


fn is_pcapng(buf: &[u8]) -> bool {

    if buf.starts_with(&PCAPNG_BYTES){
        return true;
    }

    false
}

pub fn parse_pcap_from_buffer(buf: &[u8]) -> Result<(), io::Error> {


    //receives buffer data and is true for .pcapng and false for .pcap data
    match is_pcapng(buf) {

        true =>{
            //convert to pcap file format for pcapng file tyoe
            let mut reader = match PcapNGReader::new(65536, buf) {
                Ok(v) => v,
                Err(e) =>{
                    return Err(io::Error::new(io::ErrorKind::Other, format!("error reading data: {:?}", e)))
                },
            };




            //cycle through form and convert to human readable pcap data using pnet library
            while let Ok((_offset, block)) = reader.next() {

                if let PcapBlockOwned::NG(inner_block) = block {

                    match inner_block {

                        Block::EnhancedPacket(epb) =>{

                            // convert pcap bytes in to layer 2 (ethernet frame) checking for
                            //IPv4 adn IPv6 packet data
                            if let Some(eth) = EthernetPacket::new(epb.data) {

                                if eth.get_ethertype() == EtherTypes::Ipv4 {

                                    if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {

                                        if ipv4.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {

                                        }
                                    }

                                }else if eth.get_ethertype() == EtherTypes::Ipv6 {

                                }


                            }
                        }

                        Block::SimplePacket(spb) =>{
                            if let Some(eth) = EthernetPacket::new(spb.data) {

                                if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {


                                }
                            }
                        }

                        _=> {}
                    }
                }

            }

        },
        false => {
            let (_, mut capture) = PcapCapture::from_file(buf)?
                .map_err(|e| io::Error::new(io::ErrorKind::Other, "error reading data"));
            while let Ok((_offset, block)) = capture.next(){

                if let PcapBlockOwned::Legacy(pkt) = block {
                    println!("{:#?}", pkt.data);
                }
            }
        }
    }

    Ok(())
}

/// gather file data from multipart form prior to sending to pcap_parse
pub async fn pcap_marshall_data(mut payload: Multipart) -> Result<Vec<u8>, PcapError> {
    let mut buffer: Vec<u8> = Vec::new();
    const MAX_FILE_SIZE: usize = 50 * 1024 * 1024; // 50 MB = 52,428,800 bytes

    while let Some(mut field) = payload.next().await {

        let mut field = field.map_err(|_|{

            PcapError::IOError

        })?;

        while let Some(chunk) = field.next().await {
            let chunk = chunk.map_err(|_|{

                PcapError::IOError

            })?;

            //check if the buffer is over the max file size limit
            if buffer.len() > MAX_FILE_SIZE {

                return Err(PcapError::FileTooLarge);
            }

            buffer.extend_from_slice(&chunk);
        }
    }

    Ok(buffer)
}



/// This function is used to determine the size of the pcap allowed based on subscription type
pub fn pcap_tier_size(user_tier_string: &str) -> usize {

        match user_tier_string {
            PAID       => PcapSizes::max_bytes(&PcapSizes::Free),
            BUSINESS   => PcapSizes::max_bytes(&PcapSizes::Business),
            ENTERPRISE => PcapSizes::max_bytes(&PcapSizes::Enterprise),
            _          => PcapSizes::max_bytes(&PcapSizes::Free),
        }

}

///upload multipart form "pcap or pcapng" into buffer
pub async fn upload_in_memory_pcap(mut payload: Multipart) -> Result<Vec<u8>, io::Error> {
    let mut buffer:Vec<u8> = Vec::new();

    while let Some(mut field) = payload.try_next().await {
        let mut field = field?;


        while let Some(chunk) = field.next().await{
            buffer.extend_from_slice(&chunk);
        }
    }

    if buffer.len() < 4 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "empty form"));
    }

    Ok(buffer)
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

            let clean_filename: String = safe_filename(filename);

            file_data.insert(String::from("filename"), clean_filename);

            //validate file type
            validate_file_type(&file_data["filename"])?;

            file_data.insert(
                String::from("filepath"),
                format!("./files/{}.pcap", &file_data["filename"])
            );

        };

        //crate new pcap file if error return error
        let mut f : File = create_file(&file_data["filepath"])?;
        //write the bytes to pcap and check for error
        write_to_pcap(&mut f, &mut field, &file_data["filepath"], max_pcap_size).await?;

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
        let result : Result<(), PcapError> = file.write_all(&chunk).map_err(|_| PcapError::IOError);

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





