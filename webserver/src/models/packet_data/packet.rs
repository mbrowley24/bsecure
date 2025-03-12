use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{
    Serialize,
    Deserialize,
};


#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id              : Option<ObjectId>,
    pub timestamp       : DateTime,
    pub packet_type     : String,
    pub src_ip          : String,
    pub dst_ip          : String,
    pub src_mac         : String,
    pub dst_mac         : String,
    pub src_port        : u16,
    pub dst_port        : u16,
    pub seq_num         : u16,
    pub packet_length   : u16,
    pub packet_protocol : String,
    pub protocol        : String,

}