use crate::config::load_config;

/// Returns the alphabet with letters swapped
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let result = swap(vec![('A', 'F'), ('U', 'J')]);
        assert_eq!(result, "FBCDEAGHIUKLMNOPQRSTJVWXYZ");
        let result = swap(vec![]);
        assert_eq!(result, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
}
