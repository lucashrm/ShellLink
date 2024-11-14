use std::{env};
use std::process::exit;
use shell_link::config::{Config, ConnexionMode};

use shell_link::connexion::client::client;
use shell_link::connexion::server;

fn main() {
    let args: Vec<String> = env::args().collect();

    let setup = Config::setup(args).unwrap_or_else(|e| {
        eprintln!("Setup failed: {}", e);
        exit(1);
    });

    match setup.get_mode() {
        ConnexionMode::Client => {
            let client = client::TcpConnexion::new("localhost:5444".to_string()).unwrap_or_else(|e| {
                eprintln!("Client failed: {}", e);
                exit(1);
            });
        },
        ConnexionMode::Server => {
            let server = server::server::TcpConnexion::new("0.0.0.0:5444".to_string()).unwrap_or_else(|e| {
                eprintln!("Server failed: {}", e);
                exit(1);
            });
            server.run();
        },
    }
}
