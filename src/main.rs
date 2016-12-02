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
    if args[1] == "0"{
        match recv(){
            Ok(_) => {},
            Err(msg) => println!("{}", msg)

        }
    }
    else{
        send();
    }
}
