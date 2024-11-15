pub mod server {
    use std::io::{Read, Write};
    use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
    use std::thread;
    use std::thread::{JoinHandle, Thread};
    use std::str;
    use std::sync::{Arc, Mutex};

    pub struct ThreadPool {
        handles: Vec<JoinHandle<()>>,
        client_infos: Arc<Mutex<Vec<ClientInfo>>>
    }

    pub struct ClientInfo {
        stream: TcpStream,
        peer_addr: SocketAddr,
        name: String,
    }

    pub struct TcpConnexion {
        listener: TcpListener,
        thread_pool: ThreadPool
    }

    impl TcpConnexion {
        pub fn new(ip_addr: String) -> Result<TcpConnexion, &'static str> {
            let listener = TcpListener::bind(&ip_addr);

            match listener {
                Ok(l) => Ok(TcpConnexion {
                    listener: l,
                    thread_pool: ThreadPool {
                        handles: vec![],
                        client_infos: Arc::new(Mutex::new(vec![]))
                    }
                }),
                Err(_) => Err("Couldn't bind at this addr")
            }
        }

        fn exec(mut client_info: ClientInfo) {
            let mut data = [0u8; 50];
            loop {
                match client_info.stream.read(&mut data) {
                    Ok(size) => {
                        println!("{}", str::from_utf8(&data.split_at(size).0).unwrap());
                    },
                    Err(e) => {
                        println!("An error occurred, terminating connection with {}", e);
                        client_info.stream.shutdown(Shutdown::Both).unwrap();
                        break
                    }
                }
            }
        }

        fn handle_client(&self, mut stream: TcpStream) {
            let mut data = [0u8; 50];
            match stream.read(&mut data) {
                Ok(size) => {
                    stream.write(&data[0..size]).unwrap();
                    let peer_addr = stream.peer_addr().unwrap();
                    let client_infos = ClientInfo {
                        stream,
                        peer_addr,
                        name: String::from(str::from_utf8(&data.split_at(size).0).unwrap())
                    };
                    println!("{}, {}", client_infos.peer_addr, client_infos.name);
                    let handle = thread::spawn(move || {
                        Self::exec(client_infos)
                    });
                    /*let mut infos = self.thread_pool.client_infos.lock().unwrap();
                    infos.push(client_infos);*/
                    //
                    // self.thread_pool.handles.push(handle)
                },
                Err(e) => {
                    println!("An error occurred, terminating connection with {}", e);
                    stream.shutdown(Shutdown::Both).unwrap();
                }
            }
        }

        pub fn run(&mut self) {
            for incoming in self.listener.incoming() {
                match incoming {
                    Ok(stream) => {
                        println!("New connection: {}", stream.peer_addr().unwrap());
                        self.handle_client(stream);
                    },
                    Err(e) => println!("Couldn't accept connexion: {}", e)
                }
            }
        }
    }
}