## Iryna

![https://crates.io/crates/iryna](https://img.shields.io/crates/v/iryna.svg)

- work in progress
- for learning rust


```
use std;
use std::io::Write;
use std::io::Result;
use eventloop::*;
use channel::*;
use acceptor::*;

fn main() {
    Acceptor::new()
        .worker_count(4)
        .bind("127.0.0.1", 12345)
        .handler(Box::new(|ref mut ch| {
            ch.write("Hello, world.\n".as_bytes());
            Ok(())
        }))
        .accept();
    std::thread::sleep_ms(100000);
}

```
