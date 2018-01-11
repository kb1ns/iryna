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
    selector: Arc<Poll>,
    channels: Arc<CHashMap<Token, Channel>>,
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
            Arc::clone(&self.channels),
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
                    channels.alter(e.token().clone(), |op_ch| match op_ch {
                        None => None,
                        Some(mut ch) => {
                            {
                                let on_receive = &ch.receive_handler;
                                on_receive(&mut ch.ctx);
                            }
                            Some(ch)
                        }
                    });
                }
            }
        });
    }
}
