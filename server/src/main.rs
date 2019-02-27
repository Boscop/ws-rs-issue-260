use ws::*;

fn main() {
    listen("127.0.0.1:9595", |sender| {
        Client { sender }
    }).unwrap();
}

struct Client {
    sender: ws::Sender,
}

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

    fn on_request(&mut self, _request: &Request) -> Result<Response> {
        // this works
        // return Err(Error::new(ErrorKind::Capacity, "custom error"));

        // this makes client hang at full cpu usage
        return Err(Error::new(ErrorKind::Io(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection refused")), "CONNECTION REFUSED"));
    }

    fn on_open(&mut self, h: Handshake) -> Result<()> {
        println!("connected with {:?}", h.peer_addr);
        Ok(())
    }
}
