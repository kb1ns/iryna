use std::net::SocketAddr;
use mio::*;
use mio::net::TcpStream;

pub struct Channel {
    pub channel_id: Token,
    pub remote_addr: SocketAddr,
    stream: TcpStream,
}

impl Channel {
    pub fn create(tcp: &mut TcpStream, addr: &SocketAddr, id: Token) -> Channel {
        Channel {
            channel_id: id,
            remote_addr: addr.clone(),
            //TODO error handling
            stream: tcp.try_clone().unwrap(),
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
}
