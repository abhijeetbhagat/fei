
#[ repr(u8) ]
pub enum PacketType {
    RRQ = 1,
    WRQ,
    DATA,
    ACK,
    ERROR,
}

pub static NETASCII: &'static str = "netascii";
pub static OCTET: &'static str = "octet";
