use crate::config::load_config;

#[derive(Debug)]
pub struct Plugboard {
    pub pairs: Vec<(char, char)>,
    pub modified_alphabet: String,
    pub alphabet: String,
}

impl Plugboard {
    pub fn new(pairs: Vec<(char, char)>) -> Plugboard {
        let alphabet = load_config().misc.alphabet.clone();
        let modified_alphabet = Plugboard::swap(alphabet.clone(), pairs.clone());
        Plugboard {
            alphabet,
            pairs,
            modified_alphabet,
        }
    }

    pub fn swap(alphabet: String, pairs: Vec<(char, char)>) -> String {
        let mut modified_alphabet = alphabet;
        for pair in pairs {
            let a = pair.0;
            let b = pair.1;

            modified_alphabet = modified_alphabet.replace(a, "#");
            modified_alphabet = modified_alphabet.replace(b, &a.to_string());
            modified_alphabet = modified_alphabet.replace("#", &b.to_string());
        }
        modified_alphabet.to_owned()
    }

    /// Returns the modified signal (keyboard to plugboard)
    pub fn forward(&self, signal: i32) -> i32 {
        let letter = self
            .alphabet
            .chars()
            .nth(signal.try_into().unwrap())
            .unwrap();
        self.modified_alphabet
            .find(letter)
            .unwrap()
            .try_into()
            .unwrap()
    }

    /// Returns the modified signal (plugboard to keyboard)
    pub fn backward(&self, signal: i32) -> i32 {
        let letter = self
            .modified_alphabet
            .chars()
            .nth(signal.try_into().unwrap())
            .unwrap();
        return self.alphabet.find(letter).unwrap().try_into().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let config = load_config();
        let alphabet = config.misc.alphabet;
        let result = Plugboard::swap(alphabet.clone(), vec![('A', 'F'), ('U', 'J')]);
        assert_eq!(result, "FBCDEAGHIUKLMNOPQRSTJVWXYZ");
        let result = Plugboard::swap(alphabet.clone(), vec![]);
        assert_eq!(result, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    #[test]
    fn test_forward_backward() {
        let plugboard = Plugboard::new(vec![('A', 'F'), ('U', 'J')]);

        assert_eq!(plugboard.forward(0), 5);
        assert_eq!(plugboard.backward(5), 0);
        assert_eq!(plugboard.forward(7), 7);
        assert_eq!(plugboard.backward(7), 7);
    }
}
