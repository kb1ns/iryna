use std::io::{Read, Result, Write};
use std::sync::Arc;
use std::net::SocketAddr;
use mio::*;
use mio::net::TcpStream;
use acceptor::*;

pub type Closure = Box<Fn(&mut ChanCtx) + Send + Sync>;

pub struct Channel {
    pub channel_id: Token,
    pub receive_handler: Arc<Closure>,
    pub ready_handler: Arc<Closure>,
    pub close_handler: Arc<Closure>,
    pub ctx: ChanCtx,
}

impl Channel {
    pub fn create(
        stream: &mut TcpStream,
        addr: &SocketAddr,
        id: Token,
        ready: Arc<Closure>,
        receive: Arc<Closure>,
        close: Arc<Closure>,
    ) -> Channel {
        Channel {
            channel_id: id,
            ready_handler: ready,
            receive_handler: receive,
            close_handler: close,
            ctx: ChanCtx::new(addr, stream),
        }
    }

    pub fn register(&self, selector: &Poll) {
        selector.register(
            &self.ctx.chan,
            self.channel_id,
            Ready::readable(),
            PollOpt::edge(),
        );
    }
}

pub struct ChanCtx {
    remote_addr: SocketAddr,
    chan: TcpStream,
}

impl ChanCtx {
    pub fn new(addr: &SocketAddr, stream: &mut TcpStream) -> ChanCtx {
        ChanCtx {
            remote_addr: addr.clone(),
            chan: stream.try_clone().unwrap(),
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<()> {
        self.chan.write_all(data)
    }
}
