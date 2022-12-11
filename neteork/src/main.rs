use crate::tcp_client::TcpClient;
use std::env;
mod tcp_client;

fn main() {
    println!("Hello, world!");
    // accept command line args
    let vec: Vec<String> = env::args().collect();
    if vec.len() != 2 {
        eprintln!("Please specify address and port. Usage: ./main [address]:[port]");
        std::process::exit(1);
    }
    let ptr: Box<TcpClient> = Box::new(TcpClient { vec });
}
