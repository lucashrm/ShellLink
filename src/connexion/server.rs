pub mod server {
    use std::io::{Read, Write};
    use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
    use std::thread;
    use std::str;
    use std::str::from_utf8;
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    pub struct ClientInfo {
        stream: TcpStream,
        peer_addr: SocketAddr,
        name: String,
    }

    impl ClientInfo {
        pub fn clone(client_info: &ClientInfo) -> ClientInfo {
            let stream = client_info.stream.try_clone().unwrap();
            let peer_addr = client_info.peer_addr.clone();
            let name = client_info.name.clone();

            ClientInfo {
                stream,
                peer_addr,
                name
            }
        }
    }

    pub struct TcpConnexion {
        listener: TcpListener,
    }

    impl TcpConnexion {
        pub fn new(ip_addr: String) -> Result<TcpConnexion, &'static str> {
            let listener = TcpListener::bind(&ip_addr);

            match listener {
                Ok(l) => Ok(TcpConnexion {
                    listener: l,
                }),
                Err(_) => Err("Couldn't bind at this addr")
            }
        }

        fn disconnect_client(client_info: &mut ClientInfo, clients: Arc<Mutex<Vec<ClientInfo>>>)
        {
            client_info.stream.shutdown(Shutdown::Both).unwrap();
            let mut index: usize = 0;
            for client in clients.lock().unwrap().iter().enumerate() {
                if client.1.name == client_info.name {
                    index = client.0;
                    break;
                }
            }
            clients.lock().unwrap().remove(index);
        }

        pub fn read_socket(data: &[u8], client_info: &mut ClientInfo, clients: Arc<Mutex<Vec<ClientInfo>>>) -> bool {
            match data[0] {
                1 => {
                    let size_receiver = data[2] as usize + 8;
                    let size_message = data[3] as usize + size_receiver;

                    let receiver = from_utf8(&data[8..size_receiver]).unwrap();
                    let message = from_utf8(&data[size_receiver..size_message]).unwrap();

                    let mut sent= false;
                    for client in clients.lock().unwrap().iter() {
                        println!("{:?}", client);
                        if client.name == receiver {
                            client.stream.try_clone().unwrap().write(format!("[{}]: {message}", client_info.name).as_bytes()).unwrap();
                            sent = true;
                            break;
                        }
                    }
                    if !sent {
                        client_info.stream.write("Couldn't find this user, try \"list\" command.".as_bytes()).unwrap();
                    }
                },
                2 => {
                    let mut list = String::new();
                    for client in clients.lock().unwrap().iter() {
                        list.push_str(format!("- {}\n", client.name.as_str()).as_str());
                    }
                    client_info.stream.write(list.as_bytes()).unwrap();
                },
                9 => {
                    println!("{:?}", clients);
                    Self::disconnect_client(client_info, clients);
                    return false;
                }
                _ => {}
            }
            true
        }

        fn exec(mut client_info: ClientInfo, clients: Arc<Mutex<Vec<ClientInfo>>>) {
            let mut data = [0u8; 50];
            loop {
                match client_info.stream.read(&mut data) {
                    Ok(_) => {
                        let is_running = Self::read_socket(&data, &mut client_info, Arc::clone(&clients));
                        if !is_running {
                            break
                        }
                    },
                    Err(e) => {
                        println!("An error occurred, terminating connection with {}", e);
                        Self::disconnect_client(&mut client_info, Arc::clone(&clients));
                        break
                    }
                }
            }
        }

        fn handle_client(&self, mut stream: TcpStream, clients: Arc<Mutex<Vec<ClientInfo>>>) {
            let mut data = [0u8; 50];
            match stream.read(&mut data) {
                Ok(size) => {
                    stream.write(b"Connected to the server.").unwrap();
                    let peer_addr = stream.peer_addr().unwrap();
                    let mut name = String::from(from_utf8(&data.split_at(size).0).unwrap());
                    let len = name.len();
                    name.truncate(len - 2);
                    let client_infos = ClientInfo {
                        stream: stream.try_clone().unwrap(),
                        peer_addr,
                        name
                    };
                    clients.lock().unwrap().push(ClientInfo::clone(&client_infos));
                    println!("{}, {}", client_infos.peer_addr, client_infos.name);
                    thread::spawn(move || {
                        Self::exec(client_infos, clients);
                    });
                },
                Err(e) => {
                    println!("An error occurred, terminating connection with {}", e);
                    stream.shutdown(Shutdown::Both).unwrap();
                }
            }
        }

        pub fn run(&mut self) {
            let clients: Arc<Mutex<Vec<ClientInfo>>> = Arc::new(Mutex::new(vec![]));
            for incoming in self.listener.incoming() {
                match incoming {
                    Ok(stream) => {
                        println!("New connection: {}", stream.peer_addr().unwrap());
                        self.handle_client(stream, Arc::clone(&clients));
                    },
                    Err(e) => println!("Couldn't accept connexion: {}", e)
                }
            }
        }
    }
}