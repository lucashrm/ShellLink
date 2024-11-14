pub mod client {
    use std::io::Write;
    use std::net::{TcpStream};

    pub struct TcpConnexion {
        stream: TcpStream
    }

    impl TcpConnexion {
        pub fn new(ip_addr: String) -> Result<TcpConnexion, &'static str> {
            let stream = TcpStream::connect(ip_addr);

            match stream {
                Ok(s) => Ok(TcpConnexion {
                    stream: s
                }),
                Err(_) => Err("Couldn't connect to ip address")
            }
        }

        pub fn send_message(&mut self, message: &str) {
            self.stream.write(message.as_bytes()).unwrap();
        }
    }
}