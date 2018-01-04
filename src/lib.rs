#![feature(match_default_bindings)]
mod eventloop;
mod channel;
mod acceptor;

extern crate concurrent_hashmap;
extern crate mio;

#[cfg(test)]
mod tests {

    use std;
    use std::io::Write;
    use std::io::Result;
    use eventloop::*;
    use channel::*;
    use acceptor::*;

    #[test]
    fn it_works() {
        Acceptor::new().worker_count(4).bind("127.0.0.1", 12345)
            .handler(test)
            .accept();
        std::thread::sleep_ms(100000);
    }

    fn test(ch: &mut Channel) -> Result<()> {
        ch.write("Hello, world.\n".as_bytes());
        Ok(())
    }
}
