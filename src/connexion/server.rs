pub mod server {
    use std::io::{Read, Write};
    use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
    use std::thread;
    use std::thread::JoinHandle;

    pub struct ClientInfo {
        stream: TcpStream,
        peer_addr: SocketAddr,
        name: String,
    }

    pub struct TcpConnexion {
        listener: TcpListener,
        handles: Vec<JoinHandle<()>>
    }

    impl TcpConnexion {
        pub fn new(ip_addr: String) -> Result<TcpConnexion, &'static str> {
            let listener = TcpListener::bind(&ip_addr);

            match listener {
                Ok(l) => Ok(TcpConnexion {
                    listener: l,
                    handles: vec![]
                }),
                Err(_) => Err("Couldn't bind at this addr")
            }
        }

        fn handle_client(mut stream: TcpStream) {
            let mut data = [0u8; 50];
            while match stream.read(&mut data) {
                Ok(size) => {
                    println!("{data:?} {size}");
                    stream.write(&data[0..size]).unwrap();
                    true
                },
                Err(e) => {
                    println!("An error occurred, terminating connection with {}", e);
                    stream.shutdown(Shutdown::Both).unwrap();
                    false
                }
            } {}
        }

        pub fn run(&mut self) {
            for incoming in self.listener.incoming() {
                match incoming {
                    Ok(stream) => {
                        println!("New connection: {}", stream.peer_addr().unwrap());
                        let handle = thread::spawn(move || {
                            Self::handle_client(stream)
                        });
                        self.handles.push(handle);
                    },
                    Err(e) => println!("Couldn't accept connexion: {}", e)
                }
            }
        }
    }
}