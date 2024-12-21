#![allow(non_snake_case)]
#![allow(dead_code)]

mod config;
mod enigma;

fn main() {
    println!("{}", enigma::plugboard::swap(vec![('A', 'F'), ('C', 'R')]));
}
