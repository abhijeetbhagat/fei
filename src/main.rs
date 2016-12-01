extern crate rand;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::io::{BufReader, Read, Error};
use std::fs::File;

#[ repr(u8) ]
enum PacketType{
    RRQ = 1,
    WRQ,
    DATA,
    ACK,
    ERROR
}

static NETASCII: &'static str = "netascii";
static OCTET: &'static str = "octet";

fn create_rrq_wrq_packet(p_type: PacketType, file_name: String, mode : &'static str)->Vec<u8>{
    let mut v = Vec::with_capacity(2+file_name.len()+1+mode.len()+1);
    v.push(0);
    v.push(p_type as u8);
    v.extend(file_name.as_bytes());
    v.push(0);//zero terminator
    v.extend(mode.as_bytes());
    v.push(0);//zero terminator
    v
}

struct FileStream<Read>{
    file_name : String,
    start : u64,
    end:u64,
    buf:[u8;512],
    reader : BufReader<Read>
}

impl FileStream<std::fs::File>{ //std::fs::File because we are talking concrete implementation
    fn new(file_name:String)->Option<Self>{
        let f = match std::fs::File::open(file_name.clone()){
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

impl Iterator for FileStream<std::fs::File>{
    type Item = ([u8;512], usize);
    fn next(&mut self)->Option<Self::Item>{
        let num_bytes_read = self.reader.read(&mut self.buf);
        println!("{:?}", num_bytes_read);
        let mut arr = [0;512];
        let mut i = 0usize;
        for c in self.buf.iter(){
            arr[i] = *c;
            i += 1; 
        }
        Some((arr, num_bytes_read.unwrap()))
    }
}

/*
fn create_data_packet()->Vec<u8>{

}
*/

fn recv() -> Result<(), Error> { 
    // Define the local connection information 
    let ip = Ipv4Addr::new(127, 0, 0, 1); 
    let connection = SocketAddrV4::new(ip, 6900);

    println!("rcvr waiting for incoming data");
    // Bind the socket
    let socket = try!(UdpSocket::bind(connection));
    println!("rcvr waiting for incoming data");

    // Read from the socket
    let mut buf = [0u8; 516]; //UDP packet is 516 bytes
    let (amt, src) = try!(socket.recv_from(&mut buf));
    match buf[1]{
        3 =>{
            let block_num = buf[3];
            let block_size = amt - 4;
            if block_size < 512{
                println!("recvr recvd: {:?}", &buf[0 .. amt]);
                println!("Last block of the file received");
            }
            //send ACK
            
            let mut high = 0u8;
            let mut low = 0u8;
            if block_num <= 0xFF{
                low = block_size as u8; 
            }
            else{
                low = block_num & 0xFF;
                high = block_num & 0xFF00; 
            }

            socket.send_to(&[0,PacketType::ACK as u8, high, low], src); 
        },
        _ => {}
    }

    // Print only the valid data (slice)
    println!("recv sending...");

    Ok(()) 
}


fn send() -> Result<(), Error> { 
    // Define the local connection (to send the data from) 
    let ip = Ipv4Addr::new(127, 0, 0, 1); 
    //TODO generate port # using rand
    let connection = SocketAddrV4::new(ip, 9992);

    // Bind the socket
    let socket = try!(UdpSocket::bind(connection));

    // Define the remote connection (to send the data to)
    let connection2 = SocketAddrV4::new(ip, 6900);

    // Send data via the socket
    let mut fs = FileStream::new("/home/abhi/code/rust/fei/target/debug/foo.txt".to_string()).unwrap();
    let (buf, num_bytes_read) = fs.next().unwrap();
    let mut v = Vec::with_capacity(num_bytes_read + 4);
    v.push(0);
    v.push(3);
    v.push(0);
    let mut cnt = 0;
    v.push(cnt);
    v.extend_from_slice(&buf[0..num_bytes_read]);
    try!(socket.send_to(v.as_slice(), connection2));

    let mut buf  = [0; 10];
    socket.recv_from(&mut buf);

    println!("sender recvd {:?}", buf);
    Ok(()) 
}

fn main() {

    let args : Vec<_> = std::env::args().collect();
    println!("{}", args[1]);
    if args[1] == "0"{
        println!("passed 0");
        match recv(){
            Ok(_) => {},
            Err(msg) => println!("{}", msg)

        }
    }
    else{
        println!("passed 1");
        send();
    }
}
