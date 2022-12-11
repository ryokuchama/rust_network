use std::io::{self, BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::str;
use std::time::Duration;

pub trait ITcpClient {
    fn start(&self);
}

pub struct TcpClient {
    pub vec: Vec<String>,
}

impl ITcpClient for TcpClient {
    fn start(&self) {
        let remote: SocketAddr = self.vec[1].parse().expect("Usage: ./main [address]:[port]");
        // set timeoutsec
        let mut stream: TcpStream =
            TcpStream::connect_timeout(&remote, Duration::from_secs(1)).expect("Could not connect");
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(2)))
            .unwrap();

        loop {
            let mut input = String::new();
            // read from inputs
            // unwrap means that it returns T type value from wrapped variable.
            std::io::stdin().read_line(&mut input).unwrap();
            // write into stream
            stream.write(input.as_bytes()).expect("failed to write");

            let mut reader: BufReader<&TcpStream> = std::io::BufReader::new(&stream);
            let mut buffer: Vec<u8> = Vec::new();
            reader
                .read_until(b'\n', &mut buffer)
                .expect("failed to read from the socket");
            print!(
                "{}",
                str::from_utf8(&buffer).expect("failed to convert to a String")
            );
        }
    }
}
