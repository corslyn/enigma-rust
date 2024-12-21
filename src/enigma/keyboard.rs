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
    'a'
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
}
