use plugboard::Plugboard;
use reflector::Reflector;
use rotors::Rotor;

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

    pub fn step_rotors(&mut self) {
        if self.rotorC.is_at_notch() {
            self.rotorC.step(None);
            self.rotorL.step(None);
        } else if self.rotorR.is_at_notch() {
            self.rotorC.step(None);
        }
        self.rotorR.step(None);
    }
}

#[cfg(test)]
mod tests {
    use crate::config::load_config;

    use super::*;

    #[test]
    fn test_encoding() {
        let config = load_config();
        let rotorL = Rotor::new(config.rotors.I, config.notches.I);
        let rotorC = Rotor::new(config.rotors.II, config.notches.II);
        let rotorR = Rotor::new(config.rotors.III, config.notches.III);
        let reflector = Reflector::new(config.reflectors.B);
        let plugboard = Plugboard::new(vec![('A', 'R'), ('G', 'K'), ('O', 'X')]);
        let mut enigma = Enigma::new(rotorL, rotorC, rotorR, reflector, plugboard);
        let result = enigma.encode("BONJOURXJEXSUISXNICOLASXSARKOZYXETXJXAIXLEXGRANDXPLAISIRXDEXLIREXLEXTEMPSXDESXTEMPETESXPOURXAUDIBLE".to_string());
        assert_eq!(result, "RSXZTPCQICLRVQEVMTVUXQLRNJYTEHSTAKBSVUGUYUOEUBEXTFVGFHMGRWBIATBSFFYCEUDJWREHRRIYOSHGTEAXWEIPCLZCCTR");
    }
}
