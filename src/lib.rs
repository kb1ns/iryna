#![feature(match_default_bindings)]
mod eventloop;
mod channel;
mod acceptor;

extern crate chashmap;
extern crate mio;

#[cfg(test)]
mod tests {

    use std;
    use channel::*;
    use acceptor::*;

    #[test]
    fn it_works() {
        Acceptor::new()
            .worker_count(4)
            .bind("127.0.0.1", 9098)
            .opt_nodelay(true)
            .opt_send_buf_size(4096)
            .opt_recv_buf_size(4096)
            .opt_keep_alive_ms(600000)
            .on_receive(|ref mut ch| {
                ch.write("Hello\n".as_bytes());
                ch.close();
            })
            .on_ready(|ref mut ch| {
                ch.write("Welcome.\n".as_bytes());
            })
            .accept();
    }
}
