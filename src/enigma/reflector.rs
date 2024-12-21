#[derive(Debug)]
pub struct Reflector {
    wiring: String,
}

impl Reflector {
    /// Creates a new reflector with a wiring
    pub fn new(wiring: String) -> Reflector {
        Reflector { wiring }
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
        let reflector_a_wiring = config.reflectors.A;
        let reflector_a = Reflector::new(reflector_a_wiring);
        let result = Reflector::forward(&reflector_a, 'H');
        assert_eq!(result, 23);
        let result = Reflector::forward(&reflector_a, 'Y');
        assert_eq!(result, 6);
    }

    #[test]
    fn test_backward() {
        let config = load_config();
        let reflector_a_wiring = config.reflectors.A;
        let reflector_a = Reflector::new(reflector_a_wiring);
        let result = Reflector::backward(&reflector_a, 23);
        assert_eq!(result, 'H');
        let result = Reflector::backward(&reflector_a, 6);
        assert_eq!(result, 'Y');
    }
}
