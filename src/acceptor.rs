use mio::*;
use mio::net::{TcpListener, TcpStream};


pub struct Acceptor {
    host: String,
    port: u16,
    id_alloc: usize,
    selector: Poll,
}

impl Acceptor {

    pub fn new() -> Acceptor {
        Acceptor {
            host: "0.0.0.0".to_owned(),
            port: 12345,
            id_alloc: 1,
            selector: Poll::new().unwrap(),
        }
    }

    pub fn bind(&mut self, host: &str, port: u16) -> &mut Acceptor {
        self
    }
}
