pub mod server {
    use std::net::{TcpListener};

    pub struct TcpConnexion {
        listener: TcpListener
    }

    impl TcpConnexion {
        pub fn new(ip_addr: String) -> Result<TcpConnexion, &'static str> {
            let listener = TcpListener::bind(&ip_addr);

            match listener {
                Ok(l) => Ok(TcpConnexion {
                    listener: l
                }),
                Err(_) => Err("Couldn't bind at this addr")
            }
        }

        pub fn run(&self) {
            for incoming in self.listener.incoming() {
                match incoming {
                    Ok(stream) => println!("New connection: {}", stream.peer_addr().unwrap()),
                    Err(e) => println!("Couldn't accept connexion: {}", e)
                }
            }
        }
    }
}