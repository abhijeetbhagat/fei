mod file_stream_iter;
mod tftp_specific;
mod udp_interface;

use udp_interface::*;



/*
fn create_data_packet()->Vec<u8>{

}
*/


fn send_over_wire(buf : &[u8]){
    

}

fn main() {
    let args : Vec<_> = std::env::args().collect();
    if args.len() == 1{
        println!("usage: fei <hostname> get <filename>");
        std::process::exit(0);
    }

    let mut endpoint = EndPoint::new(&*args[1]).unwrap();

    if args[1] == "0"{
        match endpoint.recv(){
            Ok(_) => {},
            Err(msg) => println!("{}", msg)

        }
    }
    else{
        endpoint.send();
    }
}
