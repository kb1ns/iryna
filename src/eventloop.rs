use std::time::Duration;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::io::{Read, Result, Write};
use std::thread;
use std::sync::{Arc, RwLock};
use chashmap::CHashMap;
use mio::*;
use mio::net::TcpStream;
use channel::*;

pub struct EventLoop {
    pub selector: Arc<Poll>,
    pub channels: Arc<CHashMap<Token, Channel>>,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop {
            selector: Arc::new(Poll::new().unwrap()),
            channels: Arc::new(CHashMap::new()),
        }
    }

    pub fn attach(
        &self,
        sock: &mut TcpStream,
        addr: &SocketAddr,
        token: Token,
        opts: HashMap<String, OptionValue>,
        ready_handler: Arc<Closure>,
        receive_handler: Arc<Closure>,
        close_handler: Arc<Closure>,
    ) {
        let mut ch = Channel::create(
            sock,
            addr,
            token,
            opts,
            ready_handler,
            receive_handler,
            close_handler,
        );
        ch.register(&self.selector);
        {
            let on_ready = &ch.ready_handler;
            on_ready(&mut ch.ctx);
        }
        //CAUTION
        self.channels.insert_new(token, ch);
    }

    pub fn run(&self) {
        let selector = Arc::clone(&self.selector);
        let channels = Arc::clone(&self.channels);
        thread::spawn(move || {
            let mut events = Events::with_capacity(1024);
            loop {
                selector.poll(&mut events, None).unwrap();
                for e in events.iter() {
                    if let Some(mut ch) = channels.remove(&e.token()) {
                        {
                            let on_receive = &ch.receive_handler;
                            on_receive(&mut ch.ctx);
                        }
                        if !ch.ctx.is_closed() {
                            //CAUTION
                            channels.insert_new(e.token(), ch);
                        }
                    }
                }
            }
        });
    }
}
