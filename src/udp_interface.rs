
extern crate rand;
// extern crate time;

// use time::PreciseTime;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use file_stream_iter::{FileStreamReader, FileStreamWriter};
use std::io::{ErrorKind, Error};
use tftp_specific::{ErrorCode, PacketType};
use std::str::FromStr;
use std::net::AddrParseError;
use std::error;
use std::str;
use std::path::Path;
use std::time::Duration;
use std::collections::HashMap;
use std::thread;
use utils;

pub struct EndPoint {
    local_connection: SocketAddrV4,
    // A server doesn't need a remote_connection
    remote_connection: Option<SocketAddrV4>,
}

impl EndPoint {
    pub fn new(remote: &str, port: &str, is_server: bool) -> Result<EndPoint, AddrParseError> {

        if is_server {
            Ok(EndPoint {
                local_connection: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6900),
                remote_connection: None,
            })
        } else {
            let ipv4 = try!(get_ip_from(remote));
            Ok(EndPoint {
                local_connection: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9990),
                remote_connection: Some(SocketAddrV4::new(ipv4, 6900)),
            })
        }
    }

    pub fn start_listen(&mut self) -> Result<(), Error> {
        // Bind the socket
        let socket = try!(UdpSocket::bind(self.local_connection));

        // Start servicing requests from a client
        // Read from the socket
        let mut buf = [0u8; 516]; //UDP packet is 516 bytes
        loop{ //processing loop; 
            let (amt, src) = try!(socket.recv_from(&mut buf));
            let mut last_blk_id = 0u16;
            match buf[1] {
                1 => {
                    // RRQ
                    println!("RRQ received");
                    // Get filename to be sent from the packet
                    let i = utils::get_terminator_position(&buf[2..amt-2]);
                    let filename = str::from_utf8(&buf[2..2+i]);
                    println!("file requested: {}", filename.unwrap());
                    // Send data via the socket
                    let mut fs = FileStreamReader::new(String::from(filename.unwrap())).unwrap();
                    let (mut buf, mut num_bytes_read) = fs.next().unwrap();
                    let v = self.create_data_packet(last_blk_id, &buf, num_bytes_read);
                    let mut time_out = 3; //3 secs
                    socket.set_read_timeout(Some(Duration::new(time_out, 0)));
                    socket.send_to(v.as_slice(), src);
                    loop {
                        // Get ACK
                        match socket.recv_from(&mut buf){
                            Ok((amt, src)) => {
                                if buf[1] == 4 {
                                    println!("ACK received");
                                    if num_bytes_read < 512{
                                        break;
                                    }
                                    //TODO verify the ACK received was for the last block id 
                                    last_blk_id += 1;
                                    let (buf, bytes_read) = fs.next().unwrap();
                                    num_bytes_read = bytes_read;
                                    let v = self.create_data_packet(last_blk_id, &buf, num_bytes_read);
                                    socket.send_to(v.as_slice(), src);
                                } 
                            },
                            Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                                time_out += 3;
                                socket.set_read_timeout(Some(Duration::new(time_out, 0)));
                            },
                            Err(e) => {

                            }
                        }
                    }
                },
                2 => {
                    //WRQ
                    println!("WRQ received");
                    let i = utils::get_terminator_position(&buf[2..]);

                    //TODO fix the vec use to copy slice from buf
                    let mut v = vec![0;i];
                    v.copy_from_slice(&buf[2..2+i]);
                    let filename = str::from_utf8(v.as_slice());
                    println!("file requested: {}", filename.unwrap());
                    let mut writer = FileStreamWriter::new(String::from(filename.unwrap())).unwrap();
                    //send ACK
                    let mut block_num = 0u16;
                    let mut time_out = 3;
                    loop{
                        let (low, high) = utils::to_bytes(block_num);

                        println!("Sending ACK");
                        socket.send_to(&[0, PacketType::ACK as u8, high as u8, low as u8], src);

                        Self::clear_buf(&mut buf);
                        match socket.recv_from(&mut buf){
                            Ok((amt, src)) => { 
                                let block_size = amt - 4;
                                writer.append(&buf[4..4 + block_size]);
                                if block_size < 512{
                                    writer.close();
                                    break;
                                }
                                block_num += 1;
                            },
                            Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                                time_out += 3;
                                socket.set_read_timeout(Some(Duration::new(time_out, 0)));
                            },  
                            Err(e) => {}  // bail out
                        } 
                    }

                },
                4 => {
                    let packet = self.create_error_packet(ErrorCode::ILLEGAL_TFTP_OPERATION, "No WRQ found for this data packet"); 
                    socket.send_to(packet.as_slice(), src); 
                },
                _ => panic!("unrecognized packet type"),
            }
        }

        Ok(())

    }

    fn create_error_packet(&mut self, error_code: ErrorCode, err_msg : &str)->Vec<u8>{
        let mut v = Vec::with_capacity(2+2+err_msg.len()+1);
        v.push(0);
        v.push(PacketType::ERROR as u8);
        v.push(0);
        v.push(error_code as u8);
        v.extend_from_slice(err_msg.as_bytes());
        v.push('\0' as u8);
        v 
    }

    fn clear_buf(buf : &mut[u8]){
        assert!(buf.len() == 516);
        for i in 0..516{
            buf[i] = 0;
        }
    }

    fn create_data_packet(&mut self, blk_id: u16, buf: &[u8], num_bytes: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity(num_bytes + 4);
        v.push(0);
        v.push(PacketType::DATA as u8);
        let (low, high) = utils::to_bytes(blk_id);
        v.push(high);
        v.push(low);
        v.extend_from_slice(&buf[0..num_bytes]);
        v
    }

    fn create_rrq_wrq_packet(&self,
                             p_type: PacketType,
                             file_name: &str,
                             mode: &'static str)
                             -> Vec<u8> {
        let mut v = Vec::with_capacity(2 + file_name.len() + 1 + mode.len() + 1);
        v.push(0);
        v.push(p_type as u8);
        v.extend(file_name.as_bytes());
        v.push('\0' as u8);
        v.extend(mode.as_bytes());
        v.push('\0' as u8);//zero terminator
        v
    }

    pub fn get(&mut self, files: &[&str], mode: &'static str) -> Result<(), Error> {
        let socket = try!(UdpSocket::bind(self.local_connection));
        let mut buf = [0; 516];
        for file in files {
            // start with a RRQ
            println!("sending RRQ request");
            let packet = self.create_rrq_wrq_packet(PacketType::RRQ, file, mode);
            try!(socket.send_to(packet.as_slice(), self.remote_connection.unwrap()));
            let path = Path::new(file);

            println!("Downloading as ./{}", path.file_name().unwrap().to_str().unwrap());
            //TODO fix downloading very file in the current dir
            let mut writer =
                FileStreamWriter::new(format!("./{}", path.file_name().unwrap().to_str().unwrap()))
                    .unwrap();

            loop {
                // get the first block of the requested file
                let (amt, src) = try!(socket.recv_from(&mut buf));
                println!("Received block");
                println!("Total bytes read {}", amt);

                match buf[1] {
                    3 => {
                        let block_size = amt - 4;
                        println!("File block size is {:?}", block_size);
                        if block_size == 0 {
                            println!("Empty block recvd");
                            writer.close();
                            break;
                        }
                        println!("Appending to file");
                        writer.append(&buf[4..4 + block_size]);
                        let block_num: u16 = 0u16 | (buf[2] as u16) << 8 | buf[3] as u16;

                        let (low, high) = utils::to_bytes(block_num);

                        // send ACK

                        println!("Sending ACK");
                        socket.send_to(&[0, PacketType::ACK as u8, high as u8, low as u8], src);

                    }
                    _ => {}
                }
                // Flush buf for next read
                // TODO fix this:
                for i in 0..516 {
                    buf[0] = 0;
                }
            }


        }

        Ok(())
    }


    pub fn put(&mut self, files: &[&str], mode: &'static str) -> Result<(), Error>{
        let socket = try!(UdpSocket::bind(self.local_connection));
        let mut buf = [0; 516];
        for file in files {
            // start with a WRQ
            println!("sending WRQ request");
            let packet = self.create_rrq_wrq_packet(PacketType::WRQ, file, mode);
            try!(socket.send_to(packet.as_slice(), self.remote_connection.unwrap()));
            let path = Path::new(file);

            println!("Uploading as ./{}", path.file_name().unwrap().to_str().unwrap());
            //TODO fix uploadin every file in the current dir of the server
            let mut reader =
                FileStreamReader::new(format!("./{}", path.file_name().unwrap().to_str().unwrap()))
                    .unwrap();
            let mut time_out = 3;
            let mut last_blk_id = 0u16;
            loop{
                match socket.recv_from(&mut buf){
                    Ok((amt, src)) => {
                        if buf[1] == 4 {
                            println!("ACK received");
                            //TODO verify the ACK received was for the last block id 
                            last_blk_id += 1;
                            let (buf, num_bytes_read) = reader.next().unwrap();
                            let v = self.create_data_packet(last_blk_id, &buf, num_bytes_read);
                            try!(socket.send_to(v.as_slice(), src));
                        } 
                    },
                    Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                        time_out += 3;
                        socket.set_read_timeout(Some(Duration::new(time_out, 0)));
                    },
                    Err(e) => {

                    }
                }
            }
        }

        Ok(())
    }
}

fn get_ip_from(ip: &str) -> Result<Ipv4Addr, AddrParseError> {
    Ipv4Addr::from_str(ip)
}
