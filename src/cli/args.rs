use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// Three rotor settings, e.g., "I II III"
    #[arg(short, long)]
    pub rotors: String,

    /// Reflector to use, e.g., "B"
    #[arg(long)]
    pub reflector: String,

    /// Plugboard settings, e.g., "AB CD EF" (optional)
    #[arg(short, long)]
    pub plugboard: Option<String>,

    /// Initial rotor positions, e.g., "A B C" (optional)
    #[arg(short, long)]
    pub key: Option<String>,

    /// The text to encode or decode
    #[arg(short, long)]
    pub text: String,
}
