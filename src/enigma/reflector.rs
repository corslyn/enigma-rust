use crate::config::load_config;

#[derive(Debug)]
pub struct Reflector {
    wiring: String,
}

impl Reflector {
    /// Creates a new reflector with a wiring
    pub fn new(wiring: String) -> Reflector {
        Reflector { wiring }
    }

    /* /// Returns the position of the letter
    pub fn forward(&self, letter: char) -> i32 {
        let wiring = &self.wiring;
        wiring.find(letter).unwrap().try_into().unwrap()
    } */

    /// Returns the modified signal
    pub fn reflect(&self, signal: i32) -> i32 {
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
    fn test_reflect() {
        let config = load_config();
        let reflector_a_wiring = config.reflectors.A;
        let reflector_a = Reflector::new(reflector_a_wiring);
        let result = Reflector::reflect(&reflector_a, 7);
        assert_eq!(result, 23);
        let result = Reflector::reflect(&reflector_a, 24);
        assert_eq!(result, 6);
    }
}
