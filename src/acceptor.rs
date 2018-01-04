use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use std::net::{IpAddr, SocketAddr};
use std::io::Result;
use mio::*;
use mio::net::{TcpListener, TcpStream};
use eventloop::*;
use channel::*;


pub struct Acceptor {
    host: String,
    port: u16,
    eventloop_group: Arc<RwLock<Vec<EventLoop>>>,
    handler: Option<fn(&Channel) -> Result<()>>,
    eventloop_count: usize,
}

impl Acceptor {
    pub fn new() -> Acceptor {
        Acceptor {
            host: "0.0.0.0".to_owned(),
            port: 12345,
            eventloop_group: Arc::new(RwLock::new(Vec::new())),
            handler: None,
            eventloop_count: 0,
        }
    }

    pub fn worker_count(&mut self, size: usize) -> &mut Acceptor {
        self.eventloop_count = size;
        self
    }

    pub fn handler(&mut self, f: fn(&Channel) -> Result<()>) -> &mut Acceptor {
        self.handler = Some(f);
        self
    }

    pub fn bind(&mut self, host: &str, port: u16) -> &mut Acceptor {
        self.host = host.to_string();
        self.port = port;
        self
    }

    pub fn shutdown(&self) {
        //need ref of eventloop_group
    }

    pub fn accept(&self) {
        let group = Arc::clone(&self.eventloop_group);
        let ip_addr = self.host.parse().unwrap();
        let sock_addr = Arc::new(SocketAddr::new(ip_addr, self.port));
        let count = Arc::new(self.eventloop_count);
        let const_count = Arc::clone(&count);
        thread::spawn(move || {
            let mut events = Events::with_capacity(1024);
            let mut ch_id: usize = 0;
            let listener = TcpListener::bind(&sock_addr).unwrap();
            let selector = Poll::new().unwrap();
            selector.register(&listener, Token(0), Ready::readable(), PollOpt::edge()).unwrap();
            //TODO processor * 2
            let const_count = Arc::try_unwrap(const_count).unwrap_or(1);
            //FIXME no necessary lock
            for _i in 0..const_count {
                let eventloop = EventLoop::new();
                eventloop.run();
                let mut g = group.write().unwrap();
                g.push(eventloop);
            }
            loop {
                match selector.poll(&mut events, None) {
                    Ok(_) => {},
                    Err(_) => {},
                }
                for _e in events.iter() {
                    let (mut sock, addr) = match listener.accept() {
                        Ok((s, a)) => (s, a),
                        Err(_) => {
                            continue;
                        }
                    };
                    let g = group.read().unwrap();
                    g[ch_id % const_count].attach(&mut sock, &addr, Token(ch_id));
                    ch_id = Acceptor::incr_id(ch_id);
                }
            }
        });
    }

    #[inline]
    fn incr_id(cur_id: usize) -> usize {
        if cur_id >= usize::max_value() {
            0
        } else {
            cur_id + 1
        }
    }
}
