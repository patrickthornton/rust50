pub mod rust50;
use std::env;

const ALPHABET_SIZE: u8 = 26;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} key", args[0]);
        return;
    }

    let key: u32 = match args[1].parse() {
        Ok(key) => key,
        Err(_) => {
            println!("Usage: {} key", args[0]);
            return;
        }
    };

    let plaintext = rust50::get_string("plaintext:  ")
        .expect("rust50 module should work when getting plaintext");

    let ciphertext = caesar(plaintext, key);

    println!("ciphertext: {}", ciphertext);
}

fn caesar(plaintext: String, key: u32) -> String {
    let key_u8 = (key % (ALPHABET_SIZE as u32)) as u8;
    let mut ciphertext = String::new();
    for c in plaintext.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8) - base;
            let ciphered_offset = (offset + key_u8) % ALPHABET_SIZE;
            ciphertext.push((base + ciphered_offset) as char);
        } else {
            ciphertext.push(c);
        }
    }
    ciphertext
}

// at this point, i've gotten lazy about testing stdin/stdout stuff,
// especially given the existence of the `rust50` module;
// i'm just testing the innards from now on, so to speak
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn et_tu() {
        let test_cases = [
            ("a", 1, "b"),
            ("barfoo", 23, "yxocll"),
            ("BARFOO", 3, "EDUIRR"),
            ("BaRFoo", 4, "FeVJss"),
            ("barfoo", 65, "onesbb"),
            ("world, say hello!", 12, "iadxp, emk tqxxa!"),
        ];

        for (plaintext, key, expected) in test_cases.iter() {
            assert_eq!(caesar(plaintext.to_string(), *key), expected.to_string());
        }
    }
}
