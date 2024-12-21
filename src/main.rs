mod config;
mod enigma;

fn main() {
    println!("{:?}", config::load_config());
    println!("{}", enigma::keyboard::forward('F'));
}
