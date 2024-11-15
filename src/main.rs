use std::{env, io};
use std::process::exit;
use shell_link::config::{Config, ConnexionMode};

use shell_link::connexion::client::client;
use shell_link::connexion::server::server;
use shell_link::gui::window;

fn main() {
    let args: Vec<String> = env::args().collect();

    let setup = Config::setup(args).unwrap_or_else(|e| {
        eprintln!("Setup failed: {}", e);
        exit(1);
    });

    match setup.get_mode() {
        ConnexionMode::Client => {
            let mut client = client::TcpConnexion::new("localhost:5444".to_string())
                .unwrap_or_else(|e| {
                    eprintln!("Client failed: {}", e);
                    exit(1);
                });
            let msg = "hello";
            println!("Sending hello");

            client.send_message(msg);
            loop {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                client.send_message(input.as_str());
            }
        },
        ConnexionMode::Server => {
            let mut server = server::TcpConnexion::new("0.0.0.0:5444".to_string())
                .unwrap_or_else(|e| {
                    eprintln!("Server failed: {}", e);
                    exit(1);
                });
            server.run();
        },
    }
}
