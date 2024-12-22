use plugboard::Plugboard;
use reflector::Reflector;
use rotors::Rotor;

use crate::config::load_config;

pub(crate) mod keyboard;
pub(crate) mod plugboard;
pub(crate) mod reflector;
pub(crate) mod rotors;

pub struct Enigma {
    rotorL: Rotor,
    rotorC: Rotor,
    rotorR: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn new(
        rotorL: Rotor,
        rotorC: Rotor,
        rotorR: Rotor,
        reflector: Reflector,
        plugboard: Plugboard,
    ) -> Enigma {
        Enigma {
            rotorL,
            rotorC,
            rotorR,
            reflector,
            plugboard,
        }
    }

    /// Encodes a string
    pub fn encode(&mut self, plaintext: String) -> String {
        let chars = plaintext.chars();
        let mut encoded = vec![];

        for character in chars {
            self.step_rotors();

            let mut signal = keyboard::forward(character);
            signal = self.plugboard.forward(signal);
            signal = self.rotorR.forward(signal);
            signal = self.rotorC.forward(signal);
            signal = self.rotorL.forward(signal);
            signal = self.reflector.reflect(signal);
            signal = self.rotorL.backward(signal);
            signal = self.rotorC.backward(signal);
            signal = self.rotorR.backward(signal);
            signal = self.plugboard.backward(signal);
            let letter = keyboard::backward(signal);
            encoded.push(letter);
        }
        String::from_iter(encoded.iter())
    }

    // Shifts rotors if they are at the notch
    pub fn step_rotors(&mut self) {
        if self.rotorC.is_at_notch() {
            self.rotorC.step(None);
            self.rotorL.step(None);
        } else if self.rotorR.is_at_notch() {
            self.rotorC.step(None);
        }
        self.rotorR.step(None);
    }

    #[doc = "Takes a 3 letter key as an argument and shifts the rotors to the associated letter\n
     ex: key = ABC
     rL_out : ABCDEF...
     rC_out : BCDEFG...
     rR_out : CDEFGH..."]
    pub fn set_start_position(&mut self, key: String) {
        self.rotorL.set_position(key.chars().nth(0).unwrap());
        self.rotorC.set_position(key.chars().nth(1).unwrap());
        self.rotorR.set_position(key.chars().nth(2).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::config::load_config;

    use super::*;

    #[test]
    fn test_encoding() {
        let config = load_config();
        let key = "ABC".to_string();
        let rotorL = Rotor::new(config.rotors.I, config.notches.I);
        let rotorC = Rotor::new(config.rotors.II, config.notches.II);
        let rotorR = Rotor::new(config.rotors.III, config.notches.III);
        let reflector = Reflector::new(config.reflectors.B);
        let plugboard = Plugboard::new(vec![('A', 'R'), ('G', 'K'), ('O', 'X')]);
        let mut enigma = Enigma::new(rotorL, rotorC, rotorR, reflector, plugboard);
        enigma.set_start_position(key);
        let result = enigma.encode("BONJOURXJEXSUISXNICOLASXSARKOZYXETXJXAIXLEXGRANDXPLAISIRXDEXLIREXLEXTEMPSXDESXTEMPETESXPOURXAUDIBLE".to_string());
        assert_eq!(result, "WRFAPIFBNQDWVOYLGZQQJLMFYLPSDNNIMIWCFKUCJUFHKLUCGSFZSPYLNZYPQBIWZUPOARJKKVCXWQDGGXZJYEETJPQTPXSCRTH");
    }
}
