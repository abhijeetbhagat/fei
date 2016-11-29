use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::io;

fn recv() -> Result<(), io::Error> { 
    // Define the local connection information 
    let ip = Ipv4Addr::new(127, 0, 0, 1); 
    let connection = SocketAddrV4::new(ip, 9991);

    // Bind the socket
    let socket = try!(UdpSocket::bind(connection));

    // Read from the socket
    let mut buf = [0; 10];
    let (amt, src) = try!(socket.recv_from(&mut buf));

    // Print only the valid data (slice)
    println!("recvr recvd: {:?}", &buf[0 .. amt]);
    println!("recv sending...");
    socket.send_to(&[5,4,3], src);

    Ok(()) 
}


fn send() -> Result<(), io::Error> { 
    // Define the local connection (to send the data from) 
    let ip = Ipv4Addr::new(127, 0, 0, 1); 
    let connection = SocketAddrV4::new(ip, 9992);

    // Bind the socket
    let socket = try!(UdpSocket::bind(connection));

    // Define the remote connection (to send the data to)
    let connection2 = SocketAddrV4::new(ip, 9991);

    // Send data via the socket
    let buf = &[0x01, 0x02, 0x03];
    try!(socket.send_to(buf, connection2));
    println!("sender sent {:?}", buf);

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
        recv();
    }
    else{
        println!("passed 1");
        send();
    }
}
