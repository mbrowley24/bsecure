use std::hash::Hash;
use std::net::Ipv4Addr;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TcpConnectionKey {
    src_ip: Ipv4Addr,
    src_port: u16,
    dst_ip: Ipv4Addr,
    dst_port: u16,
}


impl TcpConnectionKey {

    pub fn new(src_ip: Ipv4Addr, src_port: u16, dst_ip: Ipv4Addr, dst_port: u16) -> Self {

        Self{
            src_ip,
            src_port,
            dst_ip,
            dst_port,
        }
    }
    pub fn reserve(&self) -> Self {

        Self{
            src_ip   : self.dst_ip,
            src_port : self.dst_port,
            dst_ip   : self.src_ip,
            dst_port : self.src_port,
        }

    }

}