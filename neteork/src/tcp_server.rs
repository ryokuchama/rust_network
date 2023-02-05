use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

use log::debug;

pub trait IEchoTcpServer {
    fn serve(&self) -> Result<(), failure::Error>;
    fn handler(&self, mut stream: TcpStream) -> Result<(), failure::Error>;
}

pub struct EchoTcpServer {
    ip_address: String,
}

impl IEchoTcpServer for EchoTcpServer {
    fn serve(&self) -> Result<(), failure::Error> {
        let listener = TcpListener::bind(self.ip_address)?;
        // loop=while true. little bit faster than while
        // because conditional is not checked every time.
        loop {
            let (stream, _) = listener.accept()?;
            thread::spawn(move || {
                self.handler(stream)
                    .unwrap_or_else(|error| error!("{:?}", error));
            });
        }
    }

    // mut arg=mutable arg
    fn handler(&self, mut stream: TcpStream) -> Result<(), failure::Error> {
        // peer_addr: Returns the socket address of the remote peer of this TCP connection.
        println!("Handling data from {}", stream.peer_addr()?);
        let mut buffer = [0u8, 1024];
        loop {
            let nbytes = stream.read(&mut buffer)?;
            if nbytes == 0 {
                debug!("connection close");
                return Ok(());
            }
            println!("{}", str::from_utf8(&buffer[..nbytes])?);
        }
    }
}
