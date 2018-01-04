use std::time::Duration;
//use std::collections::HashMap;
use concurrent_hashmap::*;
use std::net::SocketAddr;
use std::io::Read;
use std::thread;
use std::rc::Rc;
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

    pub fn attach(&self, tcp: &mut TcpStream, addr: &SocketAddr, id: Token) {
        //TODO thread safe
        let mut channel = Channel::create(tcp, addr, id);
        //
        channel.register(&self.selector);
        //
        self.channels.insert(id, channel);
    }

    pub fn run(&self) {
        //FIXME is this thread safe?
        let channels_copy = Arc::clone(&self.channels);
        let selector_copy = Arc::clone(&self.selector);
        thread::spawn(move || {
            //TODO capacity
            let mut events = Events::with_capacity(1024);
            loop {
                //TODO timeout Duration::from_millis(500);

                selector_copy.poll(&mut events, None).unwrap();

                for e in events.iter() {
                    if let Some(mut ch) = channels_copy.find_mut(&e.token()) {
                        println!("{}", ch.get().remote_addr);
                    } else {
                        //TODO
                    }
                }
            }
        });
    }
}
