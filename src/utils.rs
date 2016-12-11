pub fn get_terminator_position(buf : &[u8]) -> usize{
    buf.iter().position(|e| *e == ('\0' as u8)).unwrap()
}


//TODO make it a macro
pub fn to_bytes(v : u16) -> (u8, u8){
    ((v & 0x00FF) as u8, (v >> 8) as u8)
}
