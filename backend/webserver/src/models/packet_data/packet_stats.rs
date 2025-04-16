use std::collections::HashMap;
use std::hash::Hash;
use std::net::{IpAddr};
use chrono::NaiveDateTime;
use pnet::datalink::MacAddr;
use pnet_packet::{
    ip::IpNextHeaderProtocol,
    ethernet::{
        EthernetPacket,
        EtherTypes,
    },
};
use pnet_packet::ipv4::Ipv4Packet;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Eq, FromRow, Hash, Serialize, Deserialize, PartialEq)]
struct FlowKey{
    src_ip   : IpAddr,
    dst_ip   : IpAddr,
    src_port : u16,
    dst_port : u16,
    syn      : bool,
    ack      : bool,
    protocol : IpNextHeaderProtocol,
}
#[derive(Debug, Eq, FromRow, Hash, Serialize, Deserialize, PartialEq)]
struct ARPKey{
    sender_ip  : IpAddr,
    target_ip  : IpAddr,
    sender_mac : MacAddr,
    target_mac : MacAddr,
}


#[derive(Debug, FromRow , Serialize, Deserialize)]
pub struct PacketStatus {

    pub arp_stats             : HashMap<ARPKey, usize>,
    pub bytes_per_protocol    : HashMap<IpNextHeaderProtocol, usize>,
    pub ipv4                  : HashMap<IpAddr, usize>,
    pub ipv6                  : HashMap<IpAddr, usize>,
    pub mac_addr_count        : usize,
    pub mac_add_top_talkers   : HashMap<MacAddr, usize>,
    pub packet_count          : usize,
    pub retransmissions_count : usize,
    pub retransmissions       : HashMap<IpAddr, usize>,
    pub top_talkers           : HashMap<IpAddr, usize>,
    pub top_conservations     : HashMap<FlowKey, usize>,
    pub total_protocols       : HashMap<IpNextHeaderProtocol, usize>,
    pub top_ports             : HashMap<u16, usize>,
    pub total_bytes           : usize,

}


impl PacketStatus {
    pub fn new() -> Self {

        Self{
            arp_stats             : HashMap::new(),
            bytes_per_protocol    : HashMap::new(),
            ipv4                  : HashMap::new(),
            ipv6                  : HashMap::new(),
            mac_addr_count        : 0,
            mac_add_top_talkers   : HashMap::new(),
            packet_count          : 0,
            retransmissions_count : 0,
            retransmissions       : HashMap::new(),
            top_talkers           : HashMap::new(),
            top_conservations     : HashMap::new(),
            total_protocols       : HashMap::new(),
            top_ports             : HashMap::new(),
            total_bytes           : 0,
        }
    }

    fn arp_stats(&self, ethernet: &EthernetPacket){

        match ethernet.get_ethertype(){

            EtherTypes::Ipv4 =>{

                if let Some(ipv4) = Ipv4Packet::new(ethernet.payload()){

                }
            }
            EtherTypes::Ipv6(eth)=>{

            }
        }



    }

    fn increment_packet_count(&mut self) {
        self.packet_count += 1;
    }


    fn mac_address_count(&mut self){

        self.mac_addr_count += 1
    }

    fn mac_address_top_talkers(&mut self, mac_addr: MacAddr){
        *self.mac_add_top_talkers.entry(mac_addr).or_insert(0) += 1
    }


    fn retransmissions_count(&mut self){

        self.retransmissions_count += 1;
    }

    fn retransmissions(&mut self, ip_addr: IpAddr){
        *self.retransmissions.entry(ip_addr).or_insert(0) += 1
    }


    fn top_ports(&mut self, port: &u16){

        *self.top_ports.entry(*port).or_insert(0) += 1;
    }

}