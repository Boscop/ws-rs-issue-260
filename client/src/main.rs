use ws::*;

fn main() {
    connect("ws://127.0.0.1:9595", |_sender| {
        println!("connect");
        Server
    }).unwrap()
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
