use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Use the cli Enigma machine
    Cli(CliArgs),

    /// Start the web server
    Server(ServerArgs),
}

#[derive(Debug, Args)]
pub struct CliArgs {
    /// Plugboard settings, e.g., "AB CD EF" (optional)
    #[arg(short, long)]
    pub plugboard: Option<String>,

    /// Initial rotor positions, e.g., "A B C" (optional)
    #[arg(short, long)]
    pub key: Option<String>,

    /// Three rotor settings, e.g., "I II III"
    #[arg(short, long)]
    pub rotors: String,

    /// Reflector to use, e.g., "B"
    #[arg(long)]
    pub reflector: String,

    /// The text to encode or decode
    #[arg(short, long)]
    pub text: String,
}

#[derive(Debug, Args)]
pub struct ServerArgs {
    /// Port to bind the server (Defaults to 8080)
    pub port: Option<u16>,
}
