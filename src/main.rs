#![allow(non_snake_case)]
#![allow(dead_code)]

mod config;
mod enigma;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{
        config::load_config,
        enigma::{keyboard, plugboard, reflector, rotors},
    };

    #[test]
    fn test_encoding() {
        let config = load_config();
        let rotor_i_wiring = config.rotors.I;
        let rotor_ii_wiring = config.rotors.II;
        let rotor_iii_wiring = config.rotors.III;

        let plugboard = vec![('A', 'R'), ('G', 'K'), ('O', 'X')];

        let letter = 'A';

        let mut signal = keyboard::forward(letter);
    }
}
