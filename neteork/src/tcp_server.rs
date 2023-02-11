use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

use log::debug;

pub trait IEchoTcpServer {
    // difference between self and Self
    // self: use for first arg of func
    // Self: type of the current object.
    fn serve(&self) -> Result<(), failure::Error>;
    fn handler(&self, stream: TcpStream) -> Result<(), failure::Error>;
}

pub struct EchoTcpServer {
    ip_address: String,
}

impl IEchoTcpServer for EchoTcpServer {
    // wait for imcoming connection with specific IP address
    fn serve(&self) -> Result<(), failure::Error> {
        // bound to specific IP address
        // it is nessesary to pass ref to bind().
        let listener = TcpListener::bind(&self.ip_address)?;
        // loop=while true. little bit faster than while
        // because conditional is not checked every time.
        loop {
            // accept incoming connection from client
            let (stream, _) = listener.accept()?;
            thread::spawn(move || {
                self
                    .handler(stream)
                    .unwrap_or_else(|error| log::error!("{:?}", error));
            });
        }
    }

    // wait for connection from client
    // and return same message on receiving the message
    // mut arg=mutable arg.
    // if trait func arg is not mut, impl func arg can be mut.
    // impl func arg is for inside of func. it is not related o interface.
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
