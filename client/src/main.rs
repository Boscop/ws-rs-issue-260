use std::env;
use std::str;
use std::thread;

use ws::*;

fn main() {
    let client_count = env::args()
        .nth(1)
        .map_or(100, |s| s.parse().expect("client_count"));
    let mut threads = vec![];
    for i in 0..client_count {
        threads.push(
            thread::Builder::new()
                .name(format!("ws_client {}", i))
                .spawn(move || {
                    println!("spawn {}", i);
                    connect("ws://127.0.0.1:9595", |_sender| {
                        println!("connect {}", i);
                        Server
                    })
                    .unwrap();
                    println!("terminating {}", i);
                })
                .unwrap(),
        );
    }
    for thread in threads {
        let _ = thread.join();
    }
}

struct Server;

impl Handler for Server {
    fn on_error(&mut self, err: Error) {
        println!("Server error {:?}", err);
    }

    fn on_shutdown(&mut self) {
        println!("Handler received WebSocket shutdown request.");
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("disconnected: code: {:?}, reason: {}", code, reason);
    }

    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        if let Some(addr) = shake.remote_addr()? {
            println!("Connection with {} now open", addr);
        }
        Ok(())
    }
}
