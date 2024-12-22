use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// The rotors to use (exactly 3)
    #[clap(required = true, num_args = 3)]
    #[arg(short, long)]
    pub rotors: Vec<String>,

    /// The text to process
    #[clap(required = true)]
    #[arg(short, long)]
    pub text: String,

    /// The key for the encryption
    #[clap(default_value = "AAA", last(false))]
    #[arg(short, long)]
    pub key: Option<String>,
}
