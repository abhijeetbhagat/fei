
#[ repr(u8) ]
pub enum PacketType{
    RRQ = 1,
    WRQ,
    DATA,
    ACK,
    ERROR
}

static NETASCII: &'static str = "netascii";
static OCTET: &'static str = "octet";
