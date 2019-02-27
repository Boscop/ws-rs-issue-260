use ws::*;

fn main() {
    listen("127.0.0.1:9595", |_sender| {
        Client
    }).unwrap();
}

use std::fmt;
use std::net::ToSocketAddrs;

pub fn listen<A, F, H>(addr: A, factory: F) -> Result<()>
where
    A: ToSocketAddrs + fmt::Debug,
    F: FnMut(ws::Sender) -> H,
    H: Handler,
{
    let ws = Builder::new()
        .with_settings(Settings {
            max_connections: 10, // any value will cause connecting clients above it to hang!
            ..Default::default()
        })
        .build(factory)?;
    ws.listen(addr)?;
    Ok(())
}

struct Client;

impl Handler for Client {
    fn on_error(&mut self, err: Error) {
        println!("Client error {:?}", err);
    }

    fn on_shutdown(&mut self) {
        println!("Handler received WebSocket shutdown request.");
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("disconnected: code: {:?}, reason: {}", code, reason);
    }

    fn on_request(&mut self, req: &Request) -> Result<Response> {
        println!("on_request {:?}", req.client_addr());
        Response::from_request(req)
    }

    fn on_open(&mut self, h: Handshake) -> Result<()> {
        println!("connected with {:?}", h.peer_addr);
        Ok(())
    }
}
