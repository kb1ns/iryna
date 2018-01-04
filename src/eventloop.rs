use std::time::Duration;
//use std::collections::HashMap;
use concurrent_hashmap::*;
use std::net::SocketAddr;
use std::io::{Read, Result};
use std::thread;
use std::sync::Arc;
use mio::*;
use mio::net::TcpStream;
use channel::Channel;

pub struct EventLoop {
    selector: Arc<Poll>,
    channels: Arc<ConcHashMap<Token, Channel>>,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop {
            selector: Arc::new(Poll::new().unwrap()),
            channels: Arc::new(ConcHashMap::<Token, Channel>::new()),
        }
    }

    pub fn attach(&self, sock: &mut TcpStream, addr: &SocketAddr, token: Token, func: Option<fn(&mut Channel)->Result<()>>) {
        let ch = Channel::create(sock, addr, token, func);
        ch.register(&self.selector);
        self.channels.insert(token, ch);
    }

    pub fn run(&self) {
        //FIXME is this thread safe?
        let channels = Arc::clone(&self.channels);
        let selector = Arc::clone(&self.selector);
        thread::spawn(move || {
            //TODO capacity
            let mut events = Events::with_capacity(1024);
            loop {
                //TODO timeout Duration::from_millis(500);
                selector.poll(&mut events, None).unwrap();
                for e in events.iter() {
                    if let Some(mut ch) = channels.find_mut(&e.token()) {
                        match ch.get().func {
                            None => {},
                            Some(f) => {f(&mut ch.get());}
                        }
                    } else {
                        //TODO
                    }
                }
            }
        });
    }
}
