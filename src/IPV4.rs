use std::fmt::format;

use byteorder;

enum Protocol {
    TCP,UDP
}
pub struct IP {
   pub src : String,
   pub dst: String,
   pub protocol:Protocol
}

impl IP {
    pub fn marshal(data : &[u8]) -> Self {
        let protocolNumber = data[9];
        let protocol = match protocolNumber {
            6 => Protocol::TCP,
            17 => Protocol::UDP,
            _ => Protocol::TCP
        };

        let srcIp = u32::from_be_bytes(data[12..16]);
        let dstIp = u32::from_be_bytes(data[16..20]);
        let src = IP::get_string(srcIp);
        let dst = IP::get_string(dstIp);

        IP { src, dst, protocol }
    }

    pub fn get_string(ip : u32) -> String {
        format!("{}.{}.{}.{}", (ip>>24)&0xff,  (ip>>16)&0xff,  (ip>>8)&0xff,  ip&0xff)
    }
}