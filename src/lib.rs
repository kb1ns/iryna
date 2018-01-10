#![feature(match_default_bindings)]
mod eventloop;
mod channel;
mod acceptor;

extern crate mio;
extern crate typemap;

#[cfg(test)]
mod tests {

    use std;
    use std::io::Write;
    use channel::*;
    use acceptor::*;

    #[test]
    fn it_works() {
        Acceptor::new()
            .worker_count(4)
            .bind("127.0.0.1", 12345)
            .opt_nodelay(true)
            .opt_send_buf_size(4096)
            .opt_recv_buf_size(4096)
            .on_receive(|ref mut ch| {
                ch.write("Hello, world.\n".as_bytes());
            })
            .on_ready(|ref mut ch| {
                ch.write("Welcome.\n".as_bytes());
            })
            .accept();
        std::thread::sleep_ms(100000);
    }
}
