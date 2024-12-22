use plugboard::Plugboard;
use reflector::Reflector;
use rotors::Rotor;

pub(crate) mod keyboard;
pub(crate) mod plugboard;
pub(crate) mod reflector;
pub(crate) mod rotors;

pub struct Enigma {
    rotor1: Rotor,
    rotor2: Rotor,
    rotor3: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn new(
        rotor1: Rotor,
        rotor2: Rotor,
        rotor3: Rotor,
        reflector: Reflector,
        plugboard: Plugboard,
    ) -> Enigma {
        Enigma {
            rotor1,
            rotor2,
            rotor3,
            reflector,
            plugboard,
        }
    }

    pub fn encode(&mut self, plaintext: String) -> String {
        let chars = plaintext.chars();
        let mut encoded = vec![];

        for character in chars {
            let mut signal = keyboard::forward(character);
            self.rotor3.step(None);
            signal = self.plugboard.forward(signal);
            signal = self.rotor3.forward(signal);
            signal = self.rotor2.forward(signal);
            signal = self.rotor1.forward(signal);
            signal = self.reflector.reflect(signal);
            signal = self.rotor1.backward(signal);
            signal = self.rotor2.backward(signal);
            signal = self.rotor3.backward(signal);
            signal = self.plugboard.backward(signal);
            let letter = keyboard::backward(signal);
            encoded.push(letter);
        }
        String::from_iter(encoded.iter())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::load_config;

    use super::*;

    #[test]
    fn test_encoding() {
        let config = load_config();
        let rotor1 = Rotor::new(config.rotors.I, config.notches.I);
        let rotor2 = Rotor::new(config.rotors.II, config.notches.II);
        let rotor3 = Rotor::new(config.rotors.III, config.notches.III);
        let reflector = Reflector::new(config.reflectors.A);
        let plugboard = Plugboard::new(vec![('A', 'R'), ('G', 'K'), ('O', 'X')]);
        let mut enigma = Enigma::new(rotor1, rotor2, rotor3, reflector, plugboard);
        let result = enigma.encode("A".to_string());

        assert_eq!(result, "N");
    }
}
