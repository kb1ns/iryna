use std::time::Duration;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::io::{Read, Result, Write};
use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use chashmap::CHashMap;
use mio::*;
use mio::net::TcpStream;
use channel::*;

pub struct EventLoop {
    pub selector: Arc<Poll>,
    pub channels: Arc<CHashMap<Token, Channel>>,
    stopped: Arc<AtomicBool>,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop {
            selector: Arc::new(Poll::new().unwrap()),
            channels: Arc::new(CHashMap::new()),
            stopped: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn attach(&self, id: usize, ch: Channel) {
        let mut channel = ch;
        channel.register(&self.selector);
        {
            let on_ready = &channel.ready_handler;
            on_ready(&mut channel.ctx);
        }
        self.channels.insert_new(Token(id), channel);
    }

    pub fn shutdown(&self) {
        self.stopped.store(true, Ordering::Relaxed);
    }

    pub fn run(&self) {
        let selector = Arc::clone(&self.selector);
        let channels = Arc::clone(&self.channels);
        let stopped = Arc::clone(&self.stopped);
        thread::spawn(move || {
            let mut events = Events::with_capacity(1024);
            while !stopped.load(Ordering::Relaxed) {
                selector.poll(&mut events, None).unwrap();
                for e in events.iter() {
                    if let Some(mut ch) = channels.remove(&e.token()) {
                        {
                            //TODO the same with user-config
                            let mut buf: Vec<u8> = Vec::with_capacity(4096);
                            match ch.ctx.read(&mut buf) {
                                Ok(0) => {
                                    println!("close by remote peer.");
                                    ch.ctx.close();
                                }
                                Ok(n) => {
                                }
                                Err(e) => {
                                }
                            }
                            if !ch.ctx.is_closed() {
                                let on_receive = &ch.receive_handler;
                                on_receive(&mut ch.ctx, buf);
                            } else {
                                let on_close = &ch.close_handler;
                                on_close(&mut ch.ctx);
                            }
                        }
                        if !ch.ctx.is_closed() {
                            //TODO CAUTION
                            channels.insert_new(e.token(), ch);
                        }
                    }
                }
            }
        });
    }
}
