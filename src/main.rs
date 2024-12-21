mod config;
//mod enigma;

fn main() {
    println!("{:?}", config::load_config());
}
