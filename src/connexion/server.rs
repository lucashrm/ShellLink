pub mod server {
    use std::io::{Read, Write};
    use std::net::{Shutdown, TcpListener, TcpStream};
    use std::thread;

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

        fn handle_client(mut stream: TcpStream) {
            let mut data = [0u8; 50];
            while match stream.read(&mut data) {
                Ok(size) => {
                    println!("{data:?}");
                    stream.write(&data[0..size]).unwrap();
                    true
                },
                Err(_) => {
                    println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                    stream.shutdown(Shutdown::Both).unwrap();
                    false
                }
            } {}
        }

        pub fn run(self) {
            for incoming in self.listener.incoming() {
                match incoming {
                    Ok(stream) => {
                        println!("New connection: {}", stream.peer_addr().unwrap());
                        thread::spawn(move || {
                            Self::handle_client(stream)
                        });
                    },
                    Err(e) => println!("Couldn't accept connexion: {}", e)
                }
            }
        }
    }
}