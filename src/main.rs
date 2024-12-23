#![allow(non_snake_case)]
#![allow(dead_code)]

use clap::error::ErrorKind;
use clap::{Error, Parser};
use cli::args::{Cli, CliArgs, Command};
use config::load_config;
use enigma::plugboard::Plugboard;
use enigma::reflector::Reflector;
use enigma::rotors::Rotor;
use enigma::Enigma;
use server::server::run;

mod server {
    pub mod server;
}
mod config;
mod enigma;
mod cli {
    pub mod args;
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Cli(command) => {
            encode_cli(command);
            Ok(())
        }
        Command::Server(server_args) => return run(server_args).await,
    }
}

fn encode_cli(args: CliArgs) {
    let rotors: Vec<&str> = args.rotors.split_whitespace().collect();

    if rotors.len() != 3 {
        Error::new(ErrorKind::ValueValidation).exit();
    }

    let plugboard_settings = match &args.plugboard {
        Some(plugboard) => plugboard
            .split_whitespace()
            .map(|pair| {
                let chars: Vec<char> = pair.chars().collect();
                if chars.len() != 2 {
                    Error::raw(
                        ErrorKind::ValueValidation,
                        format!(
                            "Invalid plugboard pair '{}'. Each pair must have exactly 2 letters.\n",
                            pair
                        ),
                    )
                    .exit();
                }
                (chars[0], chars[1])
            })
            .collect::<Vec<(char, char)>>(),
        None => Vec::new(),
    };

    let rotor_positions = args.key.unwrap_or_else(|| "AAA".to_string());
    let reflector = args.reflector;

    let config = load_config();
    let rotorL = Rotor::new(
        config
            .rotors
            .get(rotors[0])
            .expect("Invalid rotor")
            .to_string(),
        config.notches.get(rotors[0]).unwrap().to_string(),
    );
    let rotorC = Rotor::new(
        config
            .rotors
            .get(rotors[1])
            .expect("Invalid rotor")
            .to_string(),
        config.notches.get(rotors[1]).unwrap().to_string(),
    );
    let rotorR = Rotor::new(
        config
            .rotors
            .get(rotors[2])
            .expect("Invalid rotor")
            .to_string(),
        config.notches.get(rotors[2]).unwrap().to_string(),
    );
    let reflector = Reflector::new(
        config
            .reflectors
            .get(&reflector)
            .expect("Invalid reflector")
            .to_string(),
    );
    let plugboard = Plugboard::new(plugboard_settings);
    let mut enigma = Enigma::new(rotorL, rotorC, rotorR, reflector, plugboard);
    enigma.set_start_position(rotor_positions);
    let result = enigma.encode(args.text.to_uppercase());
    println!("Encoded text: {}", result);
}
