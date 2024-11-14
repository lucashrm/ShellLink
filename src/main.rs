use std::{env};
use std::process::exit;
use shell_link::config::{Config, ConnexionMode};

fn main() {
    let args: Vec<String> = env::args().collect();

    let setup = Config::setup(args).unwrap_or_else(|e| {
        eprintln!("Setup failed: {}", e);
        exit(1);
    });

    match setup.get_mode() {
        ConnexionMode::Client => println!("Client mode on"),
        ConnexionMode::Server => println!("Server mode on"),
    }
}
