use crate::config::load_config;

#[derive(Debug, Clone)]
pub struct Rotor {
    wiring: String,
    input: String,
    notch: String,
}

impl Rotor {
    /// Creates a new rotor with a wiring and step notch
    pub fn new(wiring: String, notch: String) -> Rotor {
        let config = load_config();
        let input = config.misc.alphabet;
        Rotor {
            wiring,
            input,
            notch,
        }
    }

    /// Steps the rotor by n or by 1 if none provided
    pub fn step(&mut self, mut next_rotor: Option<&mut Rotor>) {
        // Rotate the current rotor
        Rotor::rotate_string(&mut self.input);
        Rotor::rotate_string(&mut self.wiring);

        // Check if this rotor is at its notch
        if self.is_at_notch() {
            if let Some(ref mut next) = next_rotor {
                // Step the next rotor if this rotor is at the notch
                next.step(None);
            }
        }
    }

    /// Checks if the rotor is at the notch
    pub fn is_at_notch(&self) -> bool {
        self.input.starts_with(&self.notch)
    }

    /// Rotates the string given
    fn rotate_string(s: &mut String) {
        if let Some(first) = s.chars().next() {
            let mut chars = s.chars();
            chars.next();
            *s = chars.collect::<String>() + &first.to_string();
        }
    }

    /// Returns the modified signal (rotors to reflector)
    pub fn forward(&self, signal: i32) -> i32 {
        let input = &self.input;
        let letter = self.wiring.chars().nth(signal.try_into().unwrap()).unwrap();
        input.find(letter).unwrap().try_into().unwrap()
    }

    /// Returns the modified signal (reflector to rotors)
    pub fn backward(&self, signal: i32) -> i32 {
        let input = &self.input;
        let letter = input.chars().nth(signal.try_into().unwrap()).unwrap();
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
        rotor_iii.step(None);
        assert_eq!("DFHJLCPRTXVZNYEIWGAKMUSQOB", &rotor_iii.wiring);
    }
}
