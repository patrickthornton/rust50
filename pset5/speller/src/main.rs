// reimplements the exact functionality of cs50's speller in rust,
// with one exception; this doesn't permit .txt files that aren't
// valid UTF-8 encodings, as it uses Strings. time permitting i'd
// try an implementation that just stores and checks arrays of bytes.

use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Result, Write};
use std::{env, fs::File, io};

const EXPECTED_NUMBER_OF_WORDS_IN_LARGE_DICT: usize = 143091;

const MAX_LENGTH: usize = 45;

struct Dictionary {
    words: HashSet<String>,
}

struct Spellcheck {
    words: usize,
    misspellings: usize,
}

impl Dictionary {
    fn new() -> Self {
        Dictionary {
            words: HashSet::with_capacity(EXPECTED_NUMBER_OF_WORDS_IN_LARGE_DICT),
        }
    }

    // loads dictionary into a hashset; returns the number of words in dictionary
    fn load(&mut self, dictionary: &mut impl Read) -> Result<usize> {
        let reader = BufReader::new(dictionary);
        for line in reader.lines() {
            let line = line?;
            self.words.insert(line);
        }
        Ok(self.words.len())
    }

    // spellcheck individual word
    fn check(&self, word: &str) -> bool {
        self.words.contains(&word.to_ascii_lowercase())
    }

    // attempting to comply with cs50's implementation exactly;
    // namely: they only allow alphabetic characters and non-initial apostrophes;
    // any word with a number in it is skipped up
    // until the first non-alphanumeric character is found (this means
    // something like "100's" yields a secondary word, "s");
    // otherwise the word is spellchecked.
    fn spellcheck(&self, text: &mut impl Read, output: &mut impl Write) -> Result<Spellcheck> {
        let mut word_count = 0;
        let mut misspellings_count = 0;
        let reader = BufReader::new(text);
        for line in reader.lines() {
            let line = line.expect("input file should be valid UTF-8");
            let mut words: Vec<&str> = line
                .split(|c: char| !c.is_ascii_alphanumeric() && c != '\'')
                .collect();
            while let Some(word) = words.pop() {
                // if there's a number or it's too long, ditch this word, but check for
                // potential restarts; e.g., "rust50's" should pop "s" back onto the words stack
                if word.chars().any(|c| c.is_ascii_digit()) || word.len() > MAX_LENGTH {
                    if let Some(index) = word.find(|c: char| !c.is_ascii_alphanumeric()) {
                        words.push(&word[index + 1..]);
                    }
                    continue;
                }
                // otherwise it might be a valid word

                // shave off all non-alphabetic characters except apostrophes
                let word_string: String = word
                    .chars()
                    .filter(|c| c.is_ascii_alphanumeric() || *c == '\'')
                    .collect();
                let mut word = word_string.as_str();

                // shave off initial apostrophes
                while let Some('\'') = word.chars().next() {
                    word = &word[1..];
                }

                // if word is now empty, ditch it; otherwise, it's valid
                if word.is_empty() {
                    continue;
                }

                word_count += 1;

                // check for a misspelling
                if !self.check(word) {
                    misspellings_count += 1;
                    writeln!(output, "{}", word)
                        .expect("writing to output should work for an individual misspelling");
                }
            }
        }
        Ok(Spellcheck {
            words: word_count,
            misspellings: misspellings_count,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} dictionary text", args[0]);
        return;
    }

    let mut dictionary = File::open(&args[1])
        .unwrap_or_else(|_| panic!("should be able to open dictionary file: {}", args[1]));
    let mut text = File::open(&args[2])
        .unwrap_or_else(|_| panic!("should be able to open text file: {}", args[2]));
    let mut output = io::stdout();

    speller(&mut dictionary, &mut text, &mut output)
}

fn speller(dictionary: &mut impl Read, text: &mut impl Read, output: &mut impl Write) {
    let mut dict = Dictionary::new();
    let words = match dict.load(dictionary) {
        Ok(words) => words,
        Err(e) => {
            eprintln!("error loading dictionary!: {}", e);
            return;
        }
    };
    let data = match dict.spellcheck(text, output) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("error spellchecking text!: {}", e);
            return;
        }
    };

    writeln!(output, "WORDS MISSPELLED:     {}", data.misspellings)
        .expect("writing to output should work after spellcheck");
    writeln!(output, "WORDS IN DICTIONARY:  {}", words)
        .expect("writing to output should work after spellcheck");
    writeln!(output, "WORDS IN TEXT:        {}", data.words)
        .expect("writing to output should work after spellcheck");
}

#[cfg(test)]
mod tests {
    use super::*;

    // just ensures compliance with cs50 results
    #[test]
    fn test_load() {
        let dictionary =
            &mut File::open("dictionaries/large").expect("should be able to open dictionary");
        let mut dict = Dictionary::new();
        let words = dict
            .load(dictionary)
            .expect("should be able to load dictionary");
        assert_eq!(words, EXPECTED_NUMBER_OF_WORDS_IN_LARGE_DICT);
    }

    const TEST_CASES: [(&str, usize, usize); 26] = [
        ("texts/cat.txt", 6, 0),
        (
            "texts/pneumonoultramicroscopicsilicovolcanoconiosis.txt",
            20,
            0,
        ),
        ("texts/wordsworth.txt", 158, 0),
        ("texts/constitution.txt", 7573, 30),
        ("texts/lalaland.txt", 17756, 955),
        ("texts/carroll.txt", 29758, 295),
        ("texts/her.txt", 18402, 767),
        ("texts/birdman.txt", 21798, 1179),
        ("texts/revenant.txt", 23579, 795),
        ("texts/burnett.txt", 58171, 1000),
        ("texts/mansfield.txt", 58287, 864),
        ("texts/rinehart.txt", 72574, 1178),
        ("texts/frankenstein.txt", 80527, 2451),
        ("texts/stein.txt", 89361, 2534),
        ("texts/grimm.txt", 103614, 718),
        ("texts/austen.txt", 125203, 1614),
        ("texts/wells.txt", 133588, 4177),
        ("texts/stoker.txt", 163834, 2415),
        ("texts/xueqin1.txt", 189138, 7934),
        ("texts/homer.txt", 192975, 9339),
        ("texts/federalist.txt", 196784, 935),
        ("texts/surgery.txt", 198251, 3681),
        // ("texts/xueqin2.txt", 265867, 12544),  not valid UTF-8
        ("texts/tolstoy.txt", 567967, 13008),
        ("texts/aca.txt", 376904, 17062),
        ("texts/whittier.txt", 585394, 10111),
        ("texts/shakespeare.txt", 904612, 45691),
        // ("texts/holmes.txt", 1150970, 17845),  not valid UTF-8
    ];

    #[test]
    fn test_spellcheck_load_once() {
        let dictionary =
            &mut File::open("dictionaries/large").expect("should be able to open dictionary");
        let mut dict = Dictionary::new();
        dict.load(dictionary)
            .expect("should be able to load dictionary");

        let mut output = io::sink();
        for (text, words, misspellings) in TEST_CASES.iter() {
            println!("{:?}", text);
            let text = &mut File::open(text).expect("should be able to open text");
            let data = dict
                .spellcheck(text, &mut output)
                .expect("should be able to spellcheck text");
            assert_eq!(data.words, *words);
            assert_eq!(data.misspellings, *misspellings);
        }
    }

    #[test]
    fn test_spellcheck_load_every_time() {
        let mut output = io::sink();
        for (text, words, misspellings) in TEST_CASES.iter() {
            let dictionary =
                &mut File::open("dictionaries/large").expect("should be able to open dictionary");
            let mut dict = Dictionary::new();
            dict.load(dictionary)
                .expect("should be able to load dictionary");

            println!("{:?}", text);
            let text = &mut File::open(text).expect("should be able to open text");
            let data = dict
                .spellcheck(text, &mut output)
                .expect("should be able to spellcheck text");
            assert_eq!(data.words, *words);
            assert_eq!(data.misspellings, *misspellings);
        }
    }
}
