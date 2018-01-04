use std::io::{Read, Write, Result};
use std::net::SocketAddr;
use mio::*;
use mio::net::TcpStream;

pub struct Channel {
    pub channel_id: Token,
    pub remote_addr: SocketAddr,
    stream: TcpStream,
    pub func: Option<fn(&mut Channel)->Result<()>>,
}

impl Channel {
    pub fn create(tcp: &mut TcpStream, addr: &SocketAddr, id: Token, f: Option<fn(&mut Channel)->Result<()>>) -> Channel {
        Channel {
            channel_id: id,
            remote_addr: addr.clone(),
            //TODO error handling
            stream: tcp.try_clone().unwrap(),
            func: f,
        }
    }

    pub fn register(&self, selector: &Poll) {
        selector.register(
            &self.stream,
            self.channel_id,
            Ready::readable(),
            PollOpt::edge(),
        );
    }

    pub fn write(&mut self, buf: &[u8]) {
        self.stream.write(buf);
    }
}
