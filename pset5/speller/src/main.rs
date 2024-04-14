use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Result, Write};
use std::{env, fs::File, io};

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
            words: HashSet::new(),
        }
    }

    // loads dictionary into a hashset; returns the number of words in dictionary
    fn load(&mut self, dictionary: &mut impl Read) -> Result<usize> {
        let mut reader = BufReader::new(dictionary);
        let buffer = &mut String::new();
        while reader.read_line(buffer)? != 0 {
            self.words
                .insert(buffer.trim().to_string().to_ascii_lowercase());
            buffer.clear();
        }
        Ok(self.words.len())
    }

    fn check(&self, word: &str) -> bool {
        self.words.contains(word.to_ascii_lowercase().as_str())
    }

    // attempting to comply with cs50's implementation exactly;
    // namely: they only allow alphabetic characters and apostrophes after
    // the first character; any word with a number in it is skipped up
    // until the first non-alphanumeric character is found (this means
    // something like 100's yields a word, 's, which is considered misspelled);
    // otherwise the word is spellchecked.
    // returns the number of words and the number of misspellings in the text
    fn spellcheck(&self, text: &mut impl Read, output: &mut impl Write) -> Result<Spellcheck> {
        let mut word_count = 0;
        let mut misspellings_count = 0;
        let reader = BufReader::new(text);
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => {
                    continue;
                }
            };
            let mut words: Vec<&str> = line
                .split(|c: char| !c.is_alphanumeric() && c != '\'')
                .collect();
            while let Some(word) = words.pop() {
                // if there's a number or it's too long, check for potential restarts;
                // i.e., "check50's" should pop "'s" back onto words
                if word.chars().any(|c| c.is_numeric()) || word.len() > MAX_LENGTH {
                    if let Some(index) = word.find(|c: char| !c.is_ascii_alphanumeric()) {
                        words.push(&word[index + 1..]);
                    }
                }
                // otherwise it's a valid word
                else {
                    // shave off all non-alphabetic characters except apostrophes
                    let word: String = word
                        .chars()
                        .filter(|c| c.is_alphabetic() || *c == '\'')
                        .collect();
                    if word.len() == 0 {
                        continue;
                    }

                    // shave off initial apostrophe
                    let mut word = word.as_str();
                    if word
                        .chars()
                        .next()
                        .expect("str should have at least one char")
                        == '\''
                    {
                        word = &word[1..];
                    }
                    if word.len() == 0 {
                        continue;
                    }

                    word_count += 1;

                    // check for a misspelling
                    if !self.check(word) {
                        misspellings_count += 1;
                        writeln!(output, "{}", word).expect("bad output");
                    }
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
        .expect(format!("should be able to open dictionary file: {}", args[1]).as_str());
    let mut text = File::open(&args[2])
        .expect(format!("should be able to open text file: {}", args[2]).as_str());
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
