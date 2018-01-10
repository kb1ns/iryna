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
use std::io::Write;
use eventloop::*;
use channel::*;
use acceptor::*;

fn main() {
    Acceptor::new()
        .worker_count(4)
        .bind("127.0.0.1", 12345)
        .on_receive(|ref mut ch| {
            ch.write("Hello, world.\n".as_bytes());
        })
        .on_ready(|ref mut ch| {
            ch.write("Welcome.\n".as_bytes());
        })
        .accept();
    std::thread::sleep_ms(100000);
}

```
