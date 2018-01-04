use std::thread;
use std::sync::{Arc, Mutex};
use std::net::{IpAddr, SocketAddr};
use mio::*;
use mio::net::{TcpListener, TcpStream};
use eventloop::*;


pub struct Acceptor {
    host: String,
    port: u16,
    selector: Arc<Poll>,
    eventloop_group: Option<Arc<Vec<EventLoop>>>,
    listener: Option<Arc<TcpListener>>,
}

impl Acceptor {
    pub fn new() -> Acceptor {
        Acceptor {
            host: "0.0.0.0".to_owned(),
            port: 12345,
            selector: Arc::new(Poll::new().unwrap()),
            eventloop_group: None,
            listener: None,
        }
    }

    pub fn worker_count(&mut self, size: u8) -> &mut Acceptor {
        //FIXME
        let mut group = Vec::<EventLoop>::new();
        for i in 0..size {
            let eventloop = EventLoop::new();
            eventloop.run();
            group.push(eventloop);
        }
        self.eventloop_group = Some(Arc::new(group));
        self
    }

    pub fn bind(&mut self, host: &str, port: u16) -> &mut Acceptor {
        //TODO error
        let ip_addr = host.parse().unwrap();
        let sock_addr = SocketAddr::new(ip_addr, port);
        let l = TcpListener::bind(&sock_addr).unwrap();
        self.host = host.to_string();
        self.port = port;
        let selector = Arc::clone(&self.selector);
        selector
            .register(&l, Token(0), Ready::readable(), PollOpt::edge())
            .unwrap();
        self.listener = Some(Arc::new(l));
        self
    }

    pub fn accept(&self) {
        let selector = Arc::clone(&self.selector);
        let listener = match &self.listener {
            None => panic!("No socket address to bind."),
            Some(l) => Arc::clone(&l),
        };
        let eventloop_group = match &self.eventloop_group {
            None => panic!(""),
            Some(g) => Arc::clone(&g),
        };
        thread::spawn(move || {
            //TODO
            let mut events = Events::with_capacity(1024);
            let mut ch_id: usize = 0;
            loop {
                selector.poll(&mut events, None).unwrap();
                for e in events.iter() {
                    let (mut sock, addr) = listener.accept().unwrap();
                    //TODO
                    eventloop_group[0].attach(&mut sock, &addr, Token(ch_id));
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
