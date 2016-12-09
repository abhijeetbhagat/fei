pub fn get_terminator_position(buf : &[u8]) -> usize{
    buf.iter().position(|e| *e == ('\0' as u8)).unwrap()
}


pub fn to_bytes(v : u16) -> (u8, u8){
    (v & 0x00FF, v >> 8)
}
