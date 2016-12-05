mod file_stream_iter;
mod tftp_specific;
mod stop_watch;
mod udp_interface;

use udp_interface::*;
use tftp_specific::*;
use std::io::{Write, Read};

fn main() {
    println!("This is fei. A TFTP server+client. Written by abhijeetbhagat.");
    std::io::stdout().flush().unwrap();
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 1 {
        println!("usage: fei <hostname> get <filename>");
        std::process::exit(0);
    }

    // TODO provide a shell for the user to enter commands like the tftp utility

    let mut endpoint = if args[1] == "-s" {
        EndPoint::new("127.0.0.1", "", true).unwrap()
    } else {
        EndPoint::new(&*args[1], "", false).unwrap()
    };

    if args[1] == "-s" {
        match endpoint.start_listen() {
            Ok(_) => {}
            Err(msg) => println!("{}", msg),
        }
    } else {
        loop {
            print!("fei> ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input);
            if *&input == "q\n" || *&input == "quit\n" {
                break;
            }
            endpoint.get(&["/home/abhi/code/rust/fei/target/debug/foo.txt"], OCTET);

        }
    }
}
