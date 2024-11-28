pub mod client {
    use std::{io, thread};
    use std::io::{Read, Write};
    use std::net::{TcpStream};
    use std::process::exit;
    use std::str;
    use std::sync::{Arc, Mutex};
    use crate::pad_zeroes;

    pub struct TcpConnexion {
        stream: TcpStream,
        disconnected: bool
    }

    impl TcpConnexion {
        pub fn new(ip_addr: String) -> Result<TcpConnexion, &'static str> {
            let stream = TcpStream::connect(ip_addr);
            match stream {
                Ok(s) => Ok(TcpConnexion {
                    stream: s,
                    disconnected: false
                }),
                Err(_) => Err("Couldn't connect to ip address")
            }
        }

        pub fn send_message(&mut self, receiver: &[u8], message: &[u8]) {
            let heap = [1, 2, receiver.len() as u8, message.len() as u8];
            let heap: [u8; 8] = pad_zeroes(heap);

            let mut socket= heap.to_vec();
            socket.extend_from_slice(receiver);
            socket.extend_from_slice(message);

            println!("{:?}", socket);

            self.stream.write(&socket).unwrap();
        }

        pub fn send_list(&mut self) {
            let heap = [2];
            let heap: [u8; 8] = pad_zeroes(heap);

            self.stream.write(&heap).unwrap();
        }

        pub fn read_message(&mut self) -> Result<String, ()> {
            let mut data = [0u8; 50];
            if let Ok(s) = self.stream.read(&mut data) {
                return Ok(str::from_utf8(&data.split_at(s).0).unwrap().to_string());
            }
            Err(())
        }

        pub fn set_read_non_blocking(&self) {
            self.stream.set_nonblocking(true).unwrap_or_else(|e| {
                println!("{e}");
                exit(84)
            });
        }

        pub fn shutdown(&mut self) {
            self.disconnected = true;
        }
    }

    fn wait_messages(client: Arc<Mutex<TcpConnexion>>) {
        loop {
            if client.lock().unwrap().disconnected {
                break
            }
            if let Ok(message) = client.lock().unwrap().read_message() {
                println!("{}", message);
            }
        }
    }

    fn take_input(client: Arc<Mutex<TcpConnexion>>) {
        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Failed to read instruct");

            let array: Vec<&str> = input.split_whitespace().collect();

            if array[0].contains("quit") {
                client.lock().unwrap().shutdown();
                break
            }

            match array[0] {
                "message" | "m" => {
                    if array.len() < 3 {
                        continue
                    }
                    client.lock().unwrap().send_message(array[1].as_bytes(), array[2].as_bytes());
                },
                "help" | "h" => {
                    println!("Available commands:\n- message | m [receiver] [message]: Send a message to the given receiver.\n- list | l: Print all connected users.\n\nShellLink 0.1")
                },
                "list" | "l" => {
                    client.lock().unwrap().send_list();
                }
                _ => {
                    println!("Doesn't know this command. Try \"help\" or \"h\" to get help.");
                    continue
                }
            }
        }
    }

    pub fn start() {
        println!("Type your name:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read instruct");

        if input.len() > 16 {
            println!("Name length can't exceed 16 characters.");
            start();
        }

        let mut client = TcpConnexion::new("localhost:5444".to_string()).unwrap();

        client.stream.write(input.as_bytes()).unwrap();
        let message = client.read_message().unwrap();

        client.set_read_non_blocking();

        println!("{}", message);

        let mutex_client = Arc::new(Mutex::new(client));

        let message_client = Arc::clone(&mutex_client);
        let handle = thread::spawn(move || {
           take_input(mutex_client);
        });

        wait_messages(message_client);

        handle.join().unwrap();
    }
}