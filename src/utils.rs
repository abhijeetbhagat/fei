pub fn get_terminator_position(buf : &[u8]) -> usize{
    buf.iter().position(|e| *e == ('\0' as u8)).unwrap()
}
