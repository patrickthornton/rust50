pub mod rust50;

struct TextData {
    avg_letters_per_100_words: f64,
    avg_sentences_per_100_words: f64,
}

fn main() {
    let text = rust50::get_string("Text: ").expect("rust50 module should work when getting text");

    let result = readability(text);

    println!("{}", result);
}

fn readability(text: String) -> String {
    let data = get_data(text);
    let index = coleman_liau(data);
    if index < 1. {
        "Before Grade 1".to_string()
    } else if index >= 16. {
        "Grade 16+".to_string()
    } else {
        format!("Grade {:.0}", index)
    }
}

fn get_data(text: String) -> TextData {
    let mut letters = 0;
    let mut sentences = 0;
    let mut words = 0;

    // check text for signal characters
    for c in text.chars() {
        if c.is_alphabetic() {
            letters += 1;
        } else if c == '.' || c == '!' || c == '?' {
            sentences += 1;
        } else if c.is_whitespace() {
            words += 1;
        }
    }

    // accounting for last word
    if !text.is_empty() {
        words += 1;
    }

    let avg_letters_per_100_words = (letters as f64) / (words as f64) * 100.;
    let avg_sentences_per_100_words = (sentences as f64) / (words as f64) * 100.;
    TextData {
        avg_letters_per_100_words,
        avg_sentences_per_100_words,
    }
}

fn coleman_liau(data: TextData) -> f64 {
    0.0588 * data.avg_letters_per_100_words - 0.296 * data.avg_sentences_per_100_words - 15.8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_data() {
        let text = "One. Two! Three? yi,Pp Ee!!!".to_string();
        let data = get_data(text);
        let letters = 17;
        let sentences = 6;
        let words = 5;
        assert_eq!(
            data.avg_letters_per_100_words,
            (letters as f64) / (words as f64) * 100.
        );
        assert_eq!(
            data.avg_sentences_per_100_words,
            (sentences as f64) / (words as f64) * 100.
        );
    }

    #[test]
    fn test_readability() {
        let test_cases = [
            ("One fish. Two fish. Red fish. Blue fish.", "Before Grade 1"),
            ("Would you like them here or there? I would not like them here or there. I would not like them anywhere.", "Grade 2"),
            ("Congratulations! Today is your day. You're off to Great Places! You're off and away!", "Grade 3"),
            ("Harry Potter was a highly unusual boy in many ways. For one thing, he hated the summer holidays more than any other time of year. For another, he really wanted to do his homework, but was forced to do it in secret, in the dead of the night. And he also happened to be a wizard.", "Grade 5"),
            ("There are more things in Heaven and Earth, Horatio, than are dreamt of in your philosophy.", "Grade 9"),
            ("A large class of computational problems involve the determination of properties of graphs, digraphs, integers, arrays of integers, finite families of finite sets, boolean formulas and elements of other countable domains.", "Grade 16+"),
        ];
        for (text, expected) in test_cases.iter() {
            let result = readability(text.to_string());
            assert_eq!(result, *expected);
        }
    }
}
