
#[ repr(u8) ]
pub enum PacketType {
    RRQ = 1,
    WRQ,
    DATA,
    ACK,
    ERROR,
}

pub enum ErrorCode{
    NOT_DEFINED,
    ACCESS_VIOLATION,
    DISK_FULL_OR_ALLOCATION_EXCEEDED,
    ILLEGAL_TFTP_OPERATION,
    UNKNOWN_TRANSFER_ID,
    FILE_ALREADY_EXISTS,
    NO_SUCH_USER
}

pub static NETASCII: &'static str = "netascii";
pub static OCTET: &'static str = "octet";
