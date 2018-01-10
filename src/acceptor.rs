use std::collections::HashMap;
use std::thread;
use std::clone::Clone;
use std::sync::{Arc, Mutex, RwLock};
use std::net::{IpAddr, SocketAddr};
use std::io::{Result, Write};
use mio::*;
use mio::net::{TcpListener, TcpStream};
use eventloop::*;
use channel::*;

pub struct Acceptor {
    host: String,
    port: u16,
    eventloop_group: Option<Arc<Vec<EventLoop>>>,
    receive_handler: Arc<Closure>,
    ready_handler: Arc<Closure>,
    close_handler: Arc<Closure>,
    eventloop_count: usize,
    opts: HashMap<String, OptionValue>,
}

impl Acceptor {
    pub fn new() -> Self {
        Acceptor {
            host: "0.0.0.0".to_owned(),
            port: 12345,
            eventloop_group: None,
            receive_handler: Arc::new(Box::new(|ref mut ch| {
                ch.write("Hello, world.\n".as_bytes());
            })),
            ready_handler: Arc::new(Box::new(|ref mut ch| {
                ch.write("Welcome\n".as_bytes());
            })),
            close_handler: Arc::new(Box::new(|ref mut ch| {
                ch.write("Bye\n".as_bytes());
            })),
            eventloop_count: 0,
            opts: HashMap::new(),
        }
    }

    pub fn opt_ttl_ms(&mut self, ttl: usize) -> &mut Self {
        self.opts.insert("ttl".to_owned(), OptionValue::NUMBER(ttl));
        self
    }

    pub fn opt_linger_ms(&mut self, linger: usize) -> &mut Self {
        self.opts.insert("linger".to_owned(), OptionValue::NUMBER(linger));
        self
    }

    pub fn opt_nodelay(&mut self, nodelay: bool) -> &mut Self {
        self.opts.insert("nodelay".to_owned(), OptionValue::BOOL(nodelay));
        self
    }

    pub fn opt_keepalive_ms(&mut self, keep_alive: usize) -> &mut Self {
        self.opts.insert("keep_alive".to_owned(), OptionValue::NUMBER(keep_alive));
        self
    }

    pub fn opt_recv_buf_size(&mut self, buf_size: usize) -> &mut Self {
        self.opts.insert("recv_buf_size".to_owned(), OptionValue::NUMBER(buf_size));
        self
    }

    pub fn opt_send_buf_size(&mut self, buf_size: usize) -> &mut Self {
        self.opts.insert("send_buf_size".to_owned(), OptionValue::NUMBER(buf_size));
        self
    }

    pub fn worker_count(&mut self, size: usize) -> &mut Self {
        self.eventloop_count = size;
        let mut group = Vec::<EventLoop>::new();
        for _i in 0..size {
            group.push(EventLoop::new());
        }
        self.eventloop_group = Some(Arc::new(group));
        self
    }

    pub fn on_ready<T>(&mut self, p: T) -> &mut Self
    where
        T: Fn(&mut ChanCtx) + Send + Sync + 'static,
    {
        self.close_handler = Arc::new(Box::new(p));
        self
    }

    pub fn on_receive<T>(&mut self, p: T) -> &mut Self
    where
        T: Fn(&mut ChanCtx) + Send + Sync + 'static,
    {
        self.receive_handler = Arc::new(Box::new(p));
        self
    }

    pub fn on_close<T>(&mut self, p: T) -> &mut Self
    where
        T: Fn(&mut ChanCtx) + Send + Sync + 'static,
    {
        self.close_handler = Arc::new(Box::new(p));
        self
    }

    pub fn bind(&mut self, host: &str, port: u16) -> &mut Self {
        self.host = host.to_string();
        self.port = port;
        self
    }

    pub fn terminate(&mut self) {
        //need ref of eventloop_group
    }

    pub fn accept(&self) {
        let group = match &self.eventloop_group {
            None => panic!(""),
            Some(g) => Arc::clone(&g),
        };
        let ip_addr = self.host.parse().unwrap();
        let sock_addr = Arc::new(SocketAddr::new(ip_addr, self.port));
        let const_count = self.eventloop_count;
        let receive_handler = Arc::clone(&self.receive_handler);
        let ready_handler = Arc::clone(&self.ready_handler);
        let close_handler = Arc::clone(&self.close_handler);
        let opts = self.opts.clone();
        thread::spawn(move || {
            let mut events = Events::with_capacity(1024);
            let mut ch_id: usize = 1;
            let listener = TcpListener::bind(&sock_addr).unwrap();
            let selector = Poll::new().unwrap();
            selector
                .register(&listener, Token(0), Ready::readable(), PollOpt::edge())
                .unwrap();
            for eventloop in group.iter() {
                eventloop.run();
            }
            loop {
                match selector.poll(&mut events, None) {
                    Ok(_) => {}
                    Err(_) => {}
                }
                for _e in events.iter() {
                    let (mut sock, addr) = match listener.accept() {
                        Ok((s, a)) => (s, a),
                        Err(_) => {
                            continue;
                        }
                    };
                    group[ch_id % const_count].attach(
                        &mut sock,
                        &addr,
                        Token(ch_id),
                        opts.clone(),
                        Arc::clone(&ready_handler),
                        Arc::clone(&receive_handler),
                        Arc::clone(&close_handler),
                    );
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
