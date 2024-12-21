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
        let rotorI = rotors::Rotor::new(rotor_i_wiring, config.notches.I);
        let rotorII = rotors::Rotor::new(rotor_ii_wiring, config.notches.II);
        let rotorIII = rotors::Rotor::new(rotor_iii_wiring, config.notches.III);
        println!("{:?}", rotorIII);
        let reflector = reflector::Reflector::new(config.reflectors.A);
        let plugboard = plugboard::Plugboard::new(vec![('A', 'R'), ('G', 'K'), ('O', 'X')]);

        let letter = 'A';

        let mut signal = keyboard::forward(letter);
        println!("After keyboard forward: {:?}", keyboard::backward(signal));
        signal = plugboard.forward(signal);
        println!("After plugboard forward: {:?}", keyboard::backward(signal));
        signal = rotorIII.forward(signal);
        println!("After rotor III forward: {:?}", keyboard::backward(signal));
        signal = rotorII.forward(signal);
        println!("After rotor II forward: {:?}", keyboard::backward(signal));
        signal = rotorI.forward(signal);
        println!("After rotor I forward: {:?}", keyboard::backward(signal));
        signal = reflector.reflect(signal);
        println!("After reflector: {:?}", keyboard::backward(signal));
        signal = rotorI.backward(signal);
        println!("After rotor I backward: {:?}", keyboard::backward(signal));
        signal = rotorII.backward(signal);
        println!("After rotor II backward: {:?}", keyboard::backward(signal));
        signal = rotorIII.backward(signal);
        println!("After rotor III backward: {:?}", keyboard::backward(signal));
        signal = plugboard.backward(signal);
        println!("After plugboard backward: {:?}", keyboard::backward(signal));
        let output = keyboard::backward(signal);
        println!("Final output: {:?}", output);

        assert_eq!(output, 'X');
    }
}
