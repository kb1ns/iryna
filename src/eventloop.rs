use std::time::Duration;
use std::collections::HashMap;
use concurrent_hashmap::*;
use std::net::SocketAddr;
use std::io::{Read, Result};
use std::thread;
use std::sync::{Arc, RwLock};
use std::ops::Deref;
use mio::*;
use mio::net::TcpStream;
use channel::*;

pub struct EventLoop {
    selector: Arc<Poll>,
    channels: Arc<RwLock<HashMap<Token, Channel>>>,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop {
            selector: Arc::new(Poll::new().unwrap()),
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn attach(
        &self,
        sock: &mut TcpStream,
        addr: &SocketAddr,
        token: Token,
        handler: Option<Arc<Box<FnMut(&Channel) -> Result<()> + Send + Sync>>>,
    ) {
        let ch = Channel::create(sock, addr, token, handler);
        ch.register(&self.selector);
        let mut channels = self.channels.write().unwrap();
        channels.insert(token, ch);
    }

    pub fn run(&self) {
        let selector = Arc::clone(&self.selector);
        let channels = Arc::clone(&self.channels);
        thread::spawn(move || {
            let mut events = Events::with_capacity(1024);
            loop {
                selector.poll(&mut events, None).unwrap();
                for e in events.iter() {
                    let mut channels_lock = channels.write().unwrap();
                    if let Some(mut ch) = channels_lock.get_mut(&e.token()) {
                        let closure = match &ch.handler {
                            Some(ref h) => h,
                            None => {
                                continue;
                            }
                        };
                        (*&mut closure)(&mut ch);
                    }
                }
            }
        });
    }
}
