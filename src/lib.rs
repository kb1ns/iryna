#![feature(match_default_bindings)]
mod eventloop;
mod channel;
mod acceptor;

extern crate mio;
extern crate chashmap;

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
            .opt_keep_alive_ms(10000)
            .on_receive(|ref mut ch| {
                let sbuf: String = ch.read_test();
                match sbuf.trim_right() {
                    "quit" => {
                        ch.close();
                    }
                    _ => {
                        ch.write(sbuf.as_bytes());
                    }
                }
            })
            .on_ready(|ref mut ch| {
                ch.write("Welcome.\n".as_bytes());
            })
            .accept();
        std::thread::sleep_ms(99999999999999);
    }
}
