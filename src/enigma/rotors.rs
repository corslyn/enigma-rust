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

    /// Returns the position of the letter
    pub fn forward(&self, letter: char) -> i32 {
        let wiring = &self.wiring;
        wiring.find(letter).unwrap().try_into().unwrap()
    }

    /// Returns the letters at the specified position
    pub fn backward(&self, signal: i32) -> char {
        let wiring = &self.wiring;
        wiring.chars().nth(signal.try_into().unwrap()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config;

    #[test]
    fn test_forward() {
        let config = load_config();
        let rotor_i_wiring = config.rotors.I;
        let notch_i = config.notches.I;
        let rotor_i = Rotor::new(rotor_i_wiring, notch_i);
        let result = Rotor::forward(&rotor_i, 'A');
        assert_eq!(result, 20);
        let result = Rotor::forward(&rotor_i, 'Z');
        assert_eq!(result, 9);
    }

    #[test]
    fn test_backward() {
        let config = load_config();
        let rotor_i_wiring = config.rotors.I;
        let notch_i = config.notches.I;
        let rotor_i = Rotor::new(rotor_i_wiring, notch_i);
        let result = Rotor::backward(&rotor_i, 20);
        assert_eq!(result, 'A');
        let result = Rotor::backward(&rotor_i, 9);
        assert_eq!(result, 'Z');
    }
}
