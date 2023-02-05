use crate::tcp_client::{ITcpClient, TcpClient};
use std::env;
mod tcp_client;
mod tcp_server;

fn main() {
    println!("Start");
    // accept command line args
    let vec: Vec<String> = env::args().collect();
    if vec.len() != 2 {
        eprintln!("Please specify address and port. Usage: ./main [address]:[port]");
        std::process::exit(1);
    }
    let _ptr: Box<TcpClient> = Box::new(TcpClient { vec });
    _ptr.start();
}
