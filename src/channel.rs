use std::io::{Read, Write, Result};
use std::sync::Arc;
use std::net::SocketAddr;
use mio::*;
use mio::net::TcpStream;

pub struct Channel {
    pub channel_id: Token,
    pub remote_addr: SocketAddr,
    pub handler: Option<Arc<Box<Fn(&mut TcpStream) -> Result<()> + Send + Sync>>>,
    pub stream: TcpStream,
}

impl Channel {
    pub fn create(
        tcp: &mut TcpStream,
        addr: &SocketAddr,
        id: Token,
        h: Option<Arc<Box<Fn(&mut TcpStream) -> Result<()> + Send + Sync>>>,
    ) -> Channel {
        Channel {
            channel_id: id,
            remote_addr: addr.clone(),
            handler: h,
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

    pub fn write(&mut self, buf: &[u8]) {
        self.stream.write(buf);
    }
}

pub struct ChannelContext;
