mod file_stream_iter;
mod tftp_specific;
mod stop_watch;
mod udp_interface;
mod utils;
mod command_parser;

use udp_interface::*;
use tftp_specific::*;
use std::io::{Write, Read};
use utils::*;
extern crate regex;
use regex::Regex;
use command_parser::{TFTPCommand, Parser};

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
        println!("Running fei as server");
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
            let mut p = Parser::new();
            let (command, files) = p.parse(&input).unwrap();
            assert!(command == TFTPCommand::GET);
            assert!(files.is_some());
            println!("{:?}", files);
            if files.is_some(){
                endpoint.get(&files.unwrap(), OCTET);
            }

        }
    }
}
