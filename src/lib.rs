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
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    use eventloop::*;
    use channel::*;

    #[test]
    fn it_works() {
        let bind_addr = "127.0.0.1:12345".parse().unwrap();
        let server = TcpListener::bind(&bind_addr).unwrap();
        let poll = Poll::new().unwrap();
        poll.register(&server, Token(0), Ready::readable(), PollOpt::edge()).unwrap();
        let mut evts = Events::with_capacity(1024);

        //worker eventloop dealing with IO
        let event_loop = EventLoop::new();
        event_loop.run();

        //boss: accept connection
        loop {
            poll.poll(&mut evts, None).unwrap();
            for e in evts.iter() {
                let (mut sock, addr) = server.accept().unwrap();
                //TODO generate channel_id
                event_loop.attach(&mut sock, &addr, Token(1));
            }
        }
    }
}
