use std::io::{self, BufRead, Write};

// up to you to verify whether the greedy algorithm is correct for these denominations
// (it is for 25, 10, 5, and 1, but may not be for others)
const DENOMINATIONS: [u128; 4] = [25, 10, 5, 1];
// also, yes, u128, just for fun

fn main() {
    // BufRead needed for .read_line(); .lock() provides BufRead
    let mut input = io::stdin().lock();
    let mut output = io::stdout();
    let cents =
        get_cents(&mut input, &mut output).expect("stdin/stdout should work while getting change");
    let coins = calculate_coins(cents);

    println!("{}", coins);
}

fn get_cents(input: &mut impl BufRead, output: &mut impl Write) -> io::Result<u128> {
    let cents: u128;
    loop {
        // prompt
        output.write_all(b"Change owed: ")?;
        output.flush()?;
        let mut cents_input = String::new();
        input.read_line(&mut cents_input)?;

        // verify that cents_input is a valid nonnegative integer
        cents = match cents_input.trim().parse() {
            Ok(cents) => cents,
            Err(_) => continue,
        };

        break;
    }
    Ok(cents)
}

// greedy algorithm
fn calculate_coins(mut cents: u128) -> u128 {
    let mut coins = 0;
    for denomination in DENOMINATIONS.iter() {
        coins += cents / denomination;
        cents %= denomination;
    }
    coins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cents() {
        let mut input: &[u8] = b"-1\nWho do you think I am?\n17\n";
        let mut output: Vec<u8> = Vec::new();
        let height = get_cents(&mut input, &mut output)
            .expect("stdin/stdout should work while getting cents");

        let expected = b"Change owed: Change owed: Change owed: ";
        assert_eq!(output, expected);
        assert_eq!(height, 17);
    }

    // i'm going to start presuming that the basic stdout-ing of results work;
    // more important to test the actual function's functionalities, so to speak
    #[test]
    fn test_calculate_coins() {
        assert_eq!(calculate_coins(0), 0);
        assert_eq!(calculate_coins(1), 1);
        assert_eq!(calculate_coins(4), 4);
        assert_eq!(calculate_coins(5), 1);
        assert_eq!(calculate_coins(24), 6);
        assert_eq!(calculate_coins(25), 1);
        assert_eq!(calculate_coins(26), 2);
        assert_eq!(calculate_coins(99), 9);
    }
}
