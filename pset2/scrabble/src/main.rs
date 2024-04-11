use std::cmp::Ordering;
pub mod rust50;

const POINTS: [u32; 26] = [
    1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
];

fn main() {
    let p1 = rust50::get_string("Player 1: ")
        .expect("cs50 module should work for getting player 1's word");
    let p2 = rust50::get_string("Player 2: ")
        .expect("cs50 module should work for getting player 2's word");

    let result = scrabble(p1, p2);

    print!("{}", result);
}

fn scrabble(p1: String, p2: String) -> String {
    let s1 = compute_score(p1);
    let s2 = compute_score(p2);

    match s1.cmp(&s2) {
        Ordering::Greater => "Player 1 wins!\n".to_owned(),
        Ordering::Less => "Player 2 wins!\n".to_owned(),
        Ordering::Equal => "Tie!\n".to_owned(),
    }
}

fn compute_score(word: String) -> u32 {
    let mut score: u32 = 0;
    for mut c in word.chars() {
        if !c.is_ascii_alphabetic() {
            continue;
        }
        c.make_ascii_lowercase();
        let i = c as usize - 'a' as usize;
        score += POINTS[i];
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_score() {
        let test_cases = [
            ("././1./1./.43    \n 5./252/5.2;';.25", 0),
            ("code", 7),
            ("Oops!", 6),
        ];
        for (word, expected) in test_cases.iter() {
            let word = (*word).to_owned();
            assert_eq!(compute_score(word), *expected);
        }
    }

    #[test]
    fn test_head_to_head() {
        let test_cases = [
            ("COMPUTER", "science", "Player 1 wins!\n"),
            ("Oh,", "hai!", "Player 2 wins!\n"),
            ("pig", "dog", "Player 1 wins!\n"),
            ("figure?", "Skating!", "Player 2 wins!\n"),
            ("LETTERCASE", "lettercase", "Tie!\n"),
            ("Punctuation!?!?", "punctuation", "Tie!\n"),
            ("drawing", "illustration", "Tie!\n"),
        ];
        for (p1, p2, expected) in test_cases.iter() {
            let p1 = (*p1).to_owned();
            let p2 = (*p2).to_owned();
            assert_eq!(scrabble(p1, p2), expected.to_owned());
        }
    }
}
