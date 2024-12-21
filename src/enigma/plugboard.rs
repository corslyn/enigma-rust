use crate::config::load_config;

pub struct Plugboard {
    pub pairs: Vec<(char, char)>,
    pub modified_alphabet: String,
}

impl Plugboard {
    pub fn new(pairs: Vec<(char, char)>) -> Plugboard {
        let modified_alphabet = Plugboard::swap(pairs.clone());
        Plugboard {
            pairs,
            modified_alphabet,
        }
    }

    pub fn swap(pairs: Vec<(char, char)>) -> String {
        let config = load_config();
        let alphabet = config.misc.alphabet;
        let mut modified_alphabet = alphabet.clone();
        for pair in pairs {
            let a = pair.0;
            let b = pair.1;

            modified_alphabet = modified_alphabet.replace(a, "#");
            modified_alphabet = modified_alphabet.replace(b, &a.to_string());
            modified_alphabet = modified_alphabet.replace("#", &b.to_string());
        }
        modified_alphabet
    }

    /// Returns the position of the letter
    pub fn forward(&self, letter: char) -> i32 {
        let letters = &self.modified_alphabet;
        letters.find(letter).unwrap().try_into().unwrap()
    }

    /// Returns the letters at the specified position
    pub fn backward(&self, signal: i32) -> char {
        let letters = &self.modified_alphabet;
        letters.chars().nth(signal.try_into().unwrap()).unwrap()
    }
}
/// Returns the alphabet with letters swapped

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let result = Plugboard::swap(vec![('A', 'F'), ('U', 'J')]);
        assert_eq!(result, "FBCDEAGHIUKLMNOPQRSTJVWXYZ");
        let result = Plugboard::swap(vec![]);
        assert_eq!(result, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    #[test]
    fn test_forward_backward() {
        let plugboard = Plugboard::new(vec![('A', 'F'), ('U', 'J')]);
        assert_eq!(plugboard.forward('A'), 5);
        assert_eq!(plugboard.forward('F'), 0);
        assert_eq!(plugboard.backward(5), 'A');
        assert_eq!(plugboard.backward(0), 'F');
        assert_eq!(plugboard.forward('H'), 7);
        assert_eq!(plugboard.backward(7), 'H');
    }
}
