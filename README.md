# Iryna

[![crates](https://img.shields.io/crates/v/iryna.svg)](https://crates.io/crates/iryna)

- work in progress
- based on [mio](https://github.com/carllerche/mio)
- for learning rust

### TODO

- channel close detect
- shutdown server gracefully
- deserializer & delimiter
- initiative to send

### DEMO

An echo service

```
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
        std::thread::sleep_ms(100000);
    }

```
