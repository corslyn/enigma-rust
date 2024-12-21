use crate::config::load_config;

#[derive(Debug)]
pub struct Rotor {
    wiring: String,
    notch: String,
}

impl Rotor {
    /// Creates a new rotor with a wiring and step notch
    pub fn new(wiring: String, notch: String) -> Rotor {
        Rotor { wiring, notch }
    }

    /// Steps the rotor by one
    pub fn step(self) -> String {
        todo!("implement rotor rotation");
    }

    /// Returns the modified signal (rotors to reflector)
    pub fn forward(&self, signal: i32) -> i32 {
        let config = load_config();
        let alphabet = config.misc.alphabet;
        let letter = self.wiring.chars().nth(signal.try_into().unwrap()).unwrap();
        alphabet.find(letter).unwrap().try_into().unwrap()
    }

    /// Returns the modified signal (reflector to rotors)
    pub fn backward(&self, signal: i32) -> i32 {
        let config = load_config();
        let alphabet = config.misc.alphabet;
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
}
