#![allow(non_snake_case)]
#![allow(dead_code)]

use clap::Parser;
use cli::args::Cli;

mod config;
mod enigma;
mod cli {
    pub mod args;
}
fn main() {
    // Parse the arguments
    let args = Cli::parse();

    // Print the arguments for testing
    println!("Rotors: {:?}", args.rotors);
    println!("Text: {}", args.text);
    if let Some(key) = args.key {
        println!("Key: {}", key);
    }
}
