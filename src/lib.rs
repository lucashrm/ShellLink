use std::thread::JoinHandle;

pub mod connexion;
pub mod gui;

pub mod config {
    use crate::config::ConnexionMode::{Client, Server};

    pub enum ConnexionMode {
        Client,
        Server
    }

    pub struct Config {
        mode: ConnexionMode
    }

    impl Config {
        pub fn setup(args: Vec<String>) -> Result<Config, &'static str> {
            if args.len() < 2 {
                return Err("Not enough args")
            }

            let mode = match args[1].as_str() {
                "client" => Client,
                "server" => Server,
                _ => return Err("Invalid argument: launch mode must be client or server.")
            };

            Ok(Config {
                mode
            })
        }

        pub fn get_mode(&self) -> &ConnexionMode {
            &self.mode
        }
    }
}

pub struct Thread {
    send_handles: Vec<JoinHandle<()>>,
    receive_handles: Vec<JoinHandle<()>>
}