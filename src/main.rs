mod file_stream_iter;
mod tftp_specific;
mod udp_interface;

use udp_interface::*;
use tftp_specific::*;

fn main() {
    let args : Vec<_> = std::env::args().collect();
    if args.len() == 1{
        println!("usage: fei <hostname> get <filename>");
        std::process::exit(0);
    }

    //TODO provide a shell for the user to enter commands like the tftp utility
    let mut endpoint = EndPoint::new(&*args[1]).unwrap();

    if args[2] == "0"{
        match endpoint.start_listen(){
            Ok(_) => {},
            Err(msg) => println!("{}", msg)

        }
    }
    else{
        endpoint.get(&["/debug"], OCTET);
    }
}
