pub mod client {
    use std::{io, thread};
    use std::io::{Read, Write};
    use std::net::{TcpStream};
    use std::process::exit;
    use std::str;
    use std::sync::{Arc, Mutex};

    pub struct TcpConnexion {
        stream: TcpStream,
    }

    impl TcpConnexion {
        pub fn new(ip_addr: String) -> Result<TcpConnexion, &'static str> {
            let stream = TcpStream::connect(ip_addr);
            match stream {
                Ok(s) => Ok(TcpConnexion {
                    stream: s,
                }),
                Err(_) => Err("Couldn't connect to ip address")
            }
        }

        pub fn send_message(&mut self, message: &str) {
            self.stream.write(message.as_bytes()).unwrap();
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
    }

    fn wait_messages(client: Arc<Mutex<TcpConnexion>>) {
        loop {
            if let Ok(message) = client.lock().unwrap().read_message() {
                println!("{}", message);
            }
        }
    }

    fn take_input(client: Arc<Mutex<TcpConnexion>>) {
        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Failed to read instruct");

            client.lock().unwrap().send_message(input.as_str());
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

        client.send_message(input.as_str());
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