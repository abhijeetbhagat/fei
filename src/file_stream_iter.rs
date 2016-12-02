use std::io::{BufReader, Read, Error};
use std::fs::File;

pub struct FileStream<Read>{
    file_name : String,
    start : u64,
    end:u64,
    buf:[u8;512],
    reader : BufReader<Read>
}

impl FileStream<File>{ //std::fs::File because we are talking concrete implementation
    pub fn new(file_name:String)->Option<Self>{
        let f = match File::open(file_name.clone()){
            Ok(handle) => Some(handle),
            Err(msg) => None

        };
        if f.is_some(){
            let mut fs = FileStream{
                file_name:file_name,
                start: 0,
                end: 0,
                reader : BufReader::with_capacity(512, f.unwrap()),
                buf : [0;512]
            };
            Some(fs)
        }
        else{
            println!("cant open file");
            None
        }
    }
}

impl Iterator for FileStream<File>{
    type Item = ([u8;512], usize);
    fn next(&mut self)->Option<Self::Item>{
        let num_bytes_read = self.reader.read(&mut self.buf);
        let mut arr = [0;512];
        let mut i = 0usize;
        for c in self.buf.iter(){
            arr[i] = *c;
            i += 1; 
        }
        Some((arr, num_bytes_read.unwrap()))
    }
}
