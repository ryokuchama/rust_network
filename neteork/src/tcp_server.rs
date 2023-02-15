use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

use log::debug;

// trait: like interface
pub trait IEchoTcpServer {
    // difference between self and Self
    // self: use for first arg of func
    // Self: type of the current object.
    fn serve(&self, ip_addr: String) -> Result<(), failure::Error>;
}

pub struct EchoTcpServer {}

impl IEchoTcpServer for EchoTcpServer {
    // wait for imcoming connection with specific IP address
    fn serve(&self, ip_addr: String) -> Result<(), failure::Error> {
        // bound to specific IP address
        // it is nessesary to pass ref to bind().
        let listener = TcpListener::bind(&ip_addr)?;

        // channel is to sharing the data with multi-thread.
        // let (sender, receiver) = std::sync::mpsc::channel();
        // sender: sender.send(&self).unwrap();
        // receiver: let recv = receiver.recv().unwrap();

        // loop=while true. little bit faster than while
        // because conditional is not checked every time.
        loop {
            // accept incoming connection from client
            let (stream, _) = listener.accept()?;

            // spawn: generate new thread
            // move: enable one thread to use the data of another thread.
            //       in this case, take ownership of local variable from main thread.
            // joinhandle: if call "join()", wait for complete sub-thread.
            let result = thread::spawn(move || {
                // wait for connection from client
                // and return same message on receiving the message
                // mut arg=mutable arg.
                // if trait func arg is not mut, impl func arg can be mut.
                // impl func arg is for inside of func. it is not related o interface.

                // peer_addr: Returns the socket address of the remote peer of this TCP connection.
                // ? behind of method() means return if the method returns Err.
                println!("Handling data from {}", stream.peer_addr()?);
                let mut buffer = [0u8, 1024];
                loop {
                    let nbytes = stream.read(&mut buffer)?;
                    if nbytes == 0 {
                        debug!("connection close");
                        // Type Result
                        // Ok<T>: T was found
                        // Err<E>: E was found with error
                        return Ok(());
                    }
                    println!("{}", str::from_utf8(&buffer[..nbytes])?);
                    stream.write_all(&buffer[..nbytes]);
                }
            });
        }
    }
}
