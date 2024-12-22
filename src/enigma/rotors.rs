use crate::config::load_config;

use super::keyboard;

#[derive(Debug, Clone)]
pub struct Rotor {
    wiring: String,
    notch: String,
    alphabet: String,
}

impl Rotor {
    /// Creates a new rotor with a wiring and step notch
    pub fn new(wiring: String, notch: String) -> Rotor {
        let config = load_config();
        let alphabet = config.misc.alphabet;
        Rotor {
            wiring,
            notch,
            alphabet,
        }
    }

    /// Steps the rotor by 1
    pub fn step(&mut self, n: Option<i32>) {
        let steps = n.unwrap_or(1);

        for _ in 0..steps {
            // Rotate `alphabet`
            if !self.alphabet.is_empty() {
                let mut chars_a = self.alphabet.chars();
                if let Some(first_a) = chars_a.next() {
                    self.alphabet = chars_a.collect::<String>() + &first_a.to_string();
                }
            }

            // Rotate `wiring`
            if !self.wiring.is_empty() {
                let mut chars_w = self.wiring.chars();
                if let Some(first_w) = chars_w.next() {
                    self.wiring = chars_w.collect::<String>() + &first_w.to_string();
                }
            }
        }
    }

    /// Returns the modified signal (rotors to reflector)
    pub fn forward(&self, signal: i32) -> i32 {
        let alphabet = &self.alphabet;
        let letter = self.wiring.chars().nth(signal.try_into().unwrap()).unwrap();
        alphabet.find(letter).unwrap().try_into().unwrap()
    }

    /// Returns the modified signal (reflector to rotors)
    pub fn backward(&self, signal: i32) -> i32 {
        let alphabet = &self.alphabet;
        let letter = alphabet.chars().nth(signal.try_into().unwrap()).unwrap();
        self.wiring.find(letter).unwrap().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config;

    #[test]
    fn test_forward() {
        let config = load_config();
        let rotor_iii_wiring = config.rotors.III;
        let notch_iii = config.notches.III;
        let rotor_iii = Rotor::new(rotor_iii_wiring, notch_iii);
        let result = Rotor::forward(&rotor_iii, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_backward() {
        let config = load_config();
        let rotor_iii_wiring = config.rotors.III;
        let notch_iii = config.notches.III;
        let rotor_iii = Rotor::new(rotor_iii_wiring, notch_iii);
        let result = Rotor::backward(&rotor_iii, 1);
        assert_eq!(result, 0);
        let result = Rotor::backward(&rotor_iii, 25);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_stepping() {
        let config = load_config();
        let rotor_iii_wiring = config.rotors.III;
        let notch_iii = config.notches.III;
        let mut rotor_iii = Rotor::new(rotor_iii_wiring.clone(), notch_iii.clone());
        rotor_iii.step(Some(2));
        assert_eq!("FHJLCPRTXVZNYEIWGAKMUSQOBD", &rotor_iii.wiring);
    }
}
