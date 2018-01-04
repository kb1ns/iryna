#![feature(match_default_bindings)]
mod eventloop;
mod channel;
mod acceptor;

extern crate concurrent_hashmap;
extern crate mio;

#[cfg(test)]
mod tests {

    use std::io::Write;
    use mio::*;
    use mio::net::{TcpListener, TcpStream};
    use std;
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    use eventloop::*;
    use channel::*;
    use acceptor::*;

    #[test]
    fn it_works() {

        Acceptor::new().worker_count(4).bind("127.0.0.1", 12345).accept();

        std::thread::sleep_ms(100000);
    }
}
