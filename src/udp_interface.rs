
extern crate rand;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use file_stream_iter::FileStream;
use std::io::Error;
use tftp_specific::PacketType;
use std::str::FromStr;
use std::net::AddrParseError;
use std::error;


pub struct EndPoint{
    local_connection : SocketAddrV4,
    remote_connection : SocketAddrV4 
}

impl EndPoint{

    pub fn new(remote : &str) -> Result<EndPoint, AddrParseError>{

        let ipv4 = try!(get_ip_from(remote));
        Ok(EndPoint{
            local_connection : SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 6900),
            remote_connection : SocketAddrV4::new(ipv4, 9990) 
        })
    }


    fn start_listen(){

    }

    pub fn recv(&mut self) -> Result<(), Error> { 
        // Define the local connection information 
        //let ip = Ipv4Addr::new(127, 0, 0, 1); 
        //TODO we want to use port 69 here
        //let connection = SocketAddrV4::new(ip, 6900);

        // Bind the socket
        let socket = try!(UdpSocket::bind(self.local_connection));

        // Read from the socket
        let mut buf = [0u8; 516]; //UDP packet is 516 bytes
        let (amt, src) = try!(socket.recv_from(&mut buf));
        match buf[1]{
            3 =>{
                let block_num : u16 = 0u16 | (buf[2] as u16) << 8 |  buf[3] as u16;
                let block_size = amt - 4;
                println!("{:?}", block_size);
                if block_size < 512{
                    println!("recvr recvd: {:?}", &buf[0 .. amt]);
                    println!("Last block of the file received");
                }
                //send ACK

                let low = block_num & 0x00FF;
                let high = (block_num & 0xFF00) >> 8; 

                socket.send_to(&[0,PacketType::ACK as u8, high as u8, low as u8], src); 
            },
            _ => {}
        }


        Ok(()) 
    }

    pub fn send(&mut self) -> Result<(), Error> { 
        // Define the local connection (to send the data from) 
        //let ip = Ipv4Addr::new(127, 0, 0, 1); 
        //TODO generate port # using rand
        //let connection = SocketAddrV4::new(ip, 9992);

        // Bind the socket
        let socket = try!(UdpSocket::bind(self.local_connection));

        // Define the remote connection (to send the data to)
        //let connection2 = SocketAddrV4::new(ip, 6900);

        // Send data via the socket
        let mut fs = FileStream::new("/home/abhi/code/rust/fei/target/debug/foo.txt".to_string()).unwrap();
        let (buf, num_bytes_read) = fs.next().unwrap();
        let mut v = Vec::with_capacity(num_bytes_read + 4);
        v.push(0);
        v.push(3);
        v.push(0xFF);
        let mut cnt = 0xFF;
        v.push(cnt);
        v.extend_from_slice(&buf[0..num_bytes_read]);
        try!(socket.send_to(v.as_slice(), self.remote_connection));

        let mut buf  = [0; 10];
        socket.recv_from(&mut buf);

        println!("sender recvd {:?}", buf);
        Ok(()) 
    }

    fn send_file(&mut self, filename : &str) {

    }

    fn create_rrq_wrq_packet(&self, p_type: PacketType, file_name: &str, mode : &'static str)->Vec<u8>{
        let mut v = Vec::with_capacity(2+file_name.len()+1+mode.len()+1);
        v.push(0);
        v.push(p_type as u8);
        v.extend(file_name.as_bytes());
        v.push(0);//zero terminator
        v.extend(mode.as_bytes());
        v.push(0);//zero terminator
        v
    }

    fn get(&mut self, files : &[&str], mode : &'static str) -> Result<(), Error>{
        let socket = try!(UdpSocket::bind(self.local_connection));
        let mut buf  = [0; 516];
        for file in files{
            //start with a RRQ
            let packet = self.create_rrq_wrq_packet(PacketType::RRQ, file, mode);
            try!(socket.send_to(packet.as_slice(), self.remote_connection));

            loop{
                //get the first block of the requested file
                let (amt, src) = try!(socket.recv_from(&mut buf));
                match buf[1]{
                    3 =>{
                        let block_num : u16 = 0u16 | (buf[2] as u16) << 8 |  buf[3] as u16;
                        let block_size = amt - 4;
                        println!("{:?}", block_size);
                        if block_size < 512{
                            println!("recvr recvd: {:?}", &buf[0 .. amt]);
                            println!("Last block of the file received");
                        }
                        //send ACK

                        let low = block_num & 0x00FF;
                        let high = (block_num & 0xFF00) >> 8; 

                        socket.send_to(&[0,PacketType::ACK as u8, high as u8, low as u8], src); 
                    },
                    _ => {}
                }
                //Flush buf for next read
                //TODO fix this:
                for i in 0..516{
                    buf[0] = 0;
                }
            }


        } 

        Ok(())
    }
}

fn get_ip_from(ip : &str) -> Result<Ipv4Addr, AddrParseError>{
    Ipv4Addr::from_str(ip) 
}


