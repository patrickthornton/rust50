pub mod rust50;
use std::env;

const ALPHABET_SIZE: usize = 26;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} key", args[0]);
        return;
    }

    let key = args[1].to_owned();

    if !is_key_valid(&key) {
        println!("Key must contain 26 unique alphabetic characters.");
        return;
    }

    let plaintext = rust50::get_string("plaintext:  ")
        .expect("rust50 module should work when getting plaintext");

    let ciphertext = substitution(plaintext, key);

    println!("ciphertext: {}", ciphertext);
}

fn is_key_valid(key: &str) -> bool {
    if key.len() != ALPHABET_SIZE {
        return false;
    }

    let mut lookup_table = [false; ALPHABET_SIZE];
    for c in key.chars() {
        if !c.is_alphabetic() {
            return false;
        }

        let index = (c.to_ascii_lowercase() as u8 - b'a') as usize;
        if lookup_table[index] {
            return false;
        }
        lookup_table[index] = true;
    }

    true
}

fn substitution(plaintext: String, key: String) -> String {
    let mut ciphertext = String::new();
    let key: Vec<char> = key.chars().collect();
    for c in plaintext.chars() {
        if c.is_alphabetic() {
            let index = (c.to_ascii_lowercase() as u8 - b'a') as usize;
            let new_c = key[index];
            if c.is_uppercase() {
                ciphertext.push(new_c.to_ascii_uppercase());
            } else {
                ciphertext.push(new_c.to_ascii_lowercase());
            }
        } else {
            ciphertext.push(c);
        }
    }
    ciphertext
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_key_valid() {
        let test_cases = [
            ("abcdefghijklmnopqrstuvwxyz", true),
            ("ABCDEFGHIJKLMNOPQRSTUVWXYZ", true),
            ("ZBcDeFgHiJkLmNoPqsRTuVwXya", true),
            ("aBcDeFgHiJkLmNoPqRsTuVwXyZa", false),
            ("aBcDeFgHiJkLmNoPqRsTuVwXyZ1", false),
            ("aBcDeFgHiJkLmNoPqRsTuVwXyZ!", false),
            ("aBcDeFgHiJkLmNoPqRsTuVwXy", false),
            ("aBcDeFgHiJkLmNoPqRsTuVwXyZd", false),
        ];
        for (key, expected) in test_cases.iter() {
            assert_eq!(is_key_valid(&key.to_string()), *expected);
        }
    }

    #[test]
    fn test_substitution() {
        let test_cases = [
            ("A", "ZYXWVUTSRQPONMLKJIHGFEDCBA", "Z"),
            ("a", "ZYXWVUTSRQPONMLKJIHGFEDCBA", "z"),
            ("ABC", "NJQSUYBRXMOPFTHZVAWCGILKED", "NJQ"),
            ("XyZ", "NJQSUYBRXMOPFTHZVAWCGILKED", "KeD"),
            (
                "The quick brown fox jumps over the lazy dog",
                "dwusXNPQKEGCZFJBTLYROHiavm",
                "Rqx tokug wljif nja eozby jhxl rqx cdmv sjp",
            ),
            (
                "Shh... Don't tell!",
                "DWUSxnpqKEGCZFJBTLyrohIAVM",
                "Yqq... Sjf'r rxcc!",
            ),
        ];
        for (plaintext, key, expected) in test_cases.iter() {
            assert_eq!(
                substitution(plaintext.to_string(), key.to_string()),
                expected.to_string()
            );
        }
    }
}
