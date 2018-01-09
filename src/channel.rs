use std::io::{Read, Result, Write};
use std::sync::Arc;
use std::net::SocketAddr;
use mio::*;
use mio::net::TcpStream;
use acceptor::*;

pub type Closure = Box<Fn(&mut TcpStream) -> Result<()> + Send + Sync>;

pub struct Channel {
    pub channel_id: Token,
    pub remote_addr: SocketAddr,
    pub receive_handler: Arc<Closure>,
    pub ready_handler: Arc<Closure>,
    pub close_handler: Arc<Closure>,
    pub stream: TcpStream,
}

impl Channel {
    pub fn create(
        tcp: &mut TcpStream,
        addr: &SocketAddr,
        id: Token,
        ready: Arc<Closure>,
        receive: Arc<Closure>,
        close: Arc<Closure>,
    ) -> Channel {
        Channel {
            channel_id: id,
            remote_addr: addr.clone(),
            ready_handler: ready,
            receive_handler: receive,
            close_handler: close,
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

pub struct ChannelContext;
