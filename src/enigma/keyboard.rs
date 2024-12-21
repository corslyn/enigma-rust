use std::fs;

use crate::config::load_config;

/// Returns the position of the letter
pub fn forward(letter: char) -> i32 {
    let config = load_config();
    let alphabet = config.misc.alphabet;
    alphabet.find(letter).unwrap().try_into().unwrap()
}

/// Returns the letters at the specified position
pub fn backward(signal: i32) -> char {
    let config = load_config();
    let alphabet = config.misc.alphabet;
    alphabet.chars().nth(signal.try_into().unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward() {
        let result = forward('D');
        assert_eq!(result, 3);
        let result = forward('J');
        assert_eq!(result, 9);
    }

    #[test]
    fn test_backward() {
        let result = backward(3);
        assert_eq!(result, 'D');
        let result = backward(9);
        assert_eq!(result, 'J');
    }
}
