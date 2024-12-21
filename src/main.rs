#![allow(non_snake_case)]
#![allow(dead_code)]

mod config;
mod enigma;

fn main() {
    let config = config::load_config();
    let rotor_i_wiring = config.rotors.I;
    let notch_i = config.notches.I;
    let rotor_i = enigma::rotors::Rotor::new(rotor_i_wiring, notch_i);
    println!("{}", enigma::rotors::Rotor::forward(&rotor_i, 'A'));
}
