use std::collections::HashSet;
use std::io::{BufReader, Read, Result, Write};
use std::{env, fs::File, io};

const MAX_LENGTH: usize = 45;

const PROBABLE_MAX_TEXT_SIZE: usize = 1024 * 1024 * 10;
const PROBABLE_MAX_DICTIONARY_SIZE: usize = 1024 * 1024 * 2;

struct Dictionary<'a> {
    dict_buffer: [u8; PROBABLE_MAX_DICTIONARY_SIZE],
    words: HashSet<&'a [u8]>,
}

struct Spellcheck {
    words: usize,
    misspellings: usize,
}

impl<'a> Dictionary<'a> {
    fn new() -> Self {
        Dictionary {
            dict_buffer: [0; PROBABLE_MAX_DICTIONARY_SIZE],
            words: HashSet::new(),
        }
    }

    // loads dictionary into a hashset; returns the number of words in dictionary
    fn load(&mut self, dictionary: &mut impl Read) -> Result<usize> {
        let mut reader = BufReader::new(dictionary);

        reader.read(&mut self.dict_buffer);
        let words: Vec<&[u8]> = self.dict_buffer.split(|&c| c == b'\n').collect();
        for word in words {
            self.words.insert(word);
        }
        Ok(self.words.len())
    }

    fn check(&self, word: &[u8]) -> bool {
        let mut word = word.to_owned();
        for c in word {
            c.make_ascii_lowercase();
        }
        self.words.contains(word)
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
        let mut reader = BufReader::new(text);

        // new idea; just read the whole file into a byte array
        let mut text = [0; PROBABLE_MAX_TEXT_SIZE];
        reader.read(&mut text)?;
        let mut words: Vec<&[u8]> = text
            .split(|&c| !(c as char).is_alphanumeric() && c != b'\'')
            .collect();
        while let Some(mut word) = words.pop() {
            // if there's a number or it's too long, check for potential restarts;
            // i.e., "check50's" should pop "'s" back onto words
            if word.iter().any(|&c| (c as char).is_numeric()) || word.len() > MAX_LENGTH {
                if let Some(index) = word
                    .iter()
                    .position(|&c| !(c as char).is_ascii_alphanumeric())
                {
                    words.push(&word[index + 1..]);
                }
                continue;
            }
            // otherwise it's a valid word

            // shave off all non-alphabetic characters except apostrophes
            // word = word
            //     .iter()
            //     .filter(|&&c| (c as char).is_alphabetic() || c as char == '\'')
            //     .collect();
            // if word.len() == 0 {
            //     continue;
            // }

            // shave off initial apostrophe
            if word
                .iter()
                .next()
                .expect("str should have at least one char")
                .to_owned()
                == b'\''
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
                output.write_all(word).expect("bad output");
                output.write_all(b"\n").expect("bad output");
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
