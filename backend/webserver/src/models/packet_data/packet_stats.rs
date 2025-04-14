use std::collections::HashMap;
use std::hash::Hash;
use std::net::{IpAddr};
use chrono::NaiveDateTime;
use pnet::datalink::MacAddr;
use pnet_packet::{
    ip::IpNextHeaderProtocol,
    ethernet::{EtherTypes, EtherType}
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Eq, FromRow, Hash, Serialize, Deserialize, PartialEq)]
struct FlowKey{
    src_ip   : IpAddr,
    dst_ip   : IpAddr,
    src_port : u16,
    dst_port : u16,
    protocol : IpNextHeaderProtocol,
}

#[derive(Debug, FromRow , Serialize, Deserialize)]
pub struct PacketStatus {


    pub arp                   : HashMap<EtherType, usize>,
    pub bytes_per_protocol    : HashMap<IpNextHeaderProtocol, usize>,
    pub ipv4                  : HashMap<EtherType, usize>,
    pub ipv6                  : HashMap<EtherType, usize>,
    pub mac_addr_count        : usize,
    pub mac_add_top_talkers   : HashMap<MacAddr, usize>,
    pub packet_count          : usize,
    pub packets_retransmitted : usize,
    pub top_talkers           : HashMap<IpAddr, usize>,
    pub top_conservations     : HashMap<FlowKey, usize>,
    pub total_bytes           : usize,

}
