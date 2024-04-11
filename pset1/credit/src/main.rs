use std::{
    fmt::Display,
    io::{self, BufRead, Write},
};

#[derive(Debug, PartialEq, Eq)]
enum CardType {
    AmericanExpress(u64),
    Mastercard(u64),
    Visa(u64),
    Invalid,
}

impl CardType {
    fn from_ccn(ccn: u64) -> Self {
        let ccn_str = ccn.to_string();
        let len = ccn_str.len();
        match len {
            15 if ccn_str.starts_with("34") || ccn_str.starts_with("37") => {
                Self::AmericanExpress(ccn)
            }
            16 if {
                let first_two = ccn_str[..=1]
                    .parse::<u16>()
                    .expect("there should only be digits in the ccn at this point");
                (51..=55).contains(&first_two)
            } =>
            {
                Self::Mastercard(ccn)
            }
            13 | 16 if ccn_str.starts_with('4') => Self::Visa(ccn),
            _ => Self::Invalid,
        }
    }

    // luhn's algorithm
    fn checksum(&mut self) {
        let mut ccn = match *self {
            Self::AmericanExpress(ccn) => ccn,
            Self::Mastercard(ccn) => ccn,
            Self::Visa(ccn) => ccn,
            Self::Invalid => return,
        };

        let mut sum = 0;
        let mut on_digit_sum = true;

        loop {
            let digit = ccn % 10;
            ccn /= 10;

            if on_digit_sum {
                sum += digit;
            } else {
                let mut product = digit * 2;
                if product >= 10 {
                    product -= 9;
                }
                sum += product;
            }

            if ccn == 0 {
                break;
            }

            on_digit_sum = !on_digit_sum;
        }

        if sum % 10 != 0 {
            *self = Self::Invalid;
        }
    }
}

impl Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let card_type = match self {
            Self::AmericanExpress(_) => "AMEX\n".to_owned(),
            Self::Mastercard(_) => "MASTERCARD\n".to_owned(),
            Self::Visa(_) => "VISA\n".to_owned(),
            Self::Invalid => "INVALID\n".to_owned(),
        };
        write!(f, "{}", card_type)
    }
}

fn main() {
    let mut input = io::stdin().lock();
    let mut output = io::stdout();
    let ccn = get_ccn(&mut input, &mut output).expect("stdin/stdout should work while getting ccn");

    let mut card = CardType::from_ccn(ccn);
    card.checksum();

    output
        .write_all(card.to_string().as_bytes())
        .expect("stdout should work when outputting card type");
}

fn get_ccn(input: &mut impl BufRead, output: &mut impl Write) -> io::Result<u64> {
    let ccn: u64;
    loop {
        // prompt
        output.write_all(b"Number: ")?;
        output.flush()?;
        let mut ccn_input = String::new();
        input.read_line(&mut ccn_input)?;

        // verify that ccn_input is a valid nonnegative integer
        ccn = match ccn_input.trim().parse() {
            Ok(ccn) => ccn,
            Err(_) => continue,
        };

        break;
    }
    Ok(ccn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ccn() {
        let mut input: &[u8] = b"-1\nWhat precisely do you make of me?\n4003600000000014\n";
        let mut output: Vec<u8> = Vec::new();
        let ccn =
            get_ccn(&mut input, &mut output).expect("stdin/stdout should work while getting ccn");

        let expected = b"Number: Number: Number: ";
        assert_eq!(output, expected);
        assert_eq!(ccn, 4003600000000014);
    }

    #[test]
    fn test_from_ccn() {
        let ccn = 378282246310005;
        let card = CardType::from_ccn(ccn);
        assert_eq!(card, CardType::AmericanExpress(ccn));

        let ccn = 5555555555554444;
        let card = CardType::from_ccn(ccn);
        assert_eq!(card, CardType::Mastercard(ccn));

        let ccn = 4111111111111111;
        let card = CardType::from_ccn(ccn);
        assert_eq!(card, CardType::Visa(ccn));

        let ccn = 4062901840;
        let card = CardType::from_ccn(ccn);
        assert_eq!(card, CardType::Invalid);

        let ccn = 369421438430814;
        let card = CardType::from_ccn(ccn);
        assert_eq!(card, CardType::Invalid);
    }

    #[test]
    fn test_checksum() {
        let ccn = 371449635398431;
        let mut card = CardType::AmericanExpress(ccn);
        card.checksum();
        assert_eq!(card, CardType::AmericanExpress(ccn));

        let ccn = 5105105105105100;
        let mut card = CardType::Mastercard(ccn);
        card.checksum();
        assert_eq!(card, CardType::Mastercard(ccn));

        let ccn = 4012888888881881;
        let mut card = CardType::Visa(ccn);
        card.checksum();
        assert_eq!(card, CardType::Visa(ccn));

        let ccn = 1234567890;
        let mut card = CardType::AmericanExpress(ccn);
        card.checksum();
        assert_eq!(card, CardType::Invalid);

        let ccn = 4111111111111113;
        let mut card = CardType::Visa(ccn);
        card.checksum();
        assert_eq!(card, CardType::Invalid);

        let ccn = 4222222222223;
        let mut card = CardType::Mastercard(ccn);
        card.checksum();
        assert_eq!(card, CardType::Invalid);

        let mut card = CardType::Invalid;
        card.checksum();
        assert_eq!(card, CardType::Invalid);
    }

    #[test]
    fn test_whole_program() {
        let test_pairs = [
            ("378282246310005\n", "AMEX\n"),
            ("5105105105105100\n", "MASTERCARD\n"),
            ("4222222222222\n", "VISA\n"),
            ("5673598276138003\n", "INVALID\n"),
            ("3400000000000620\n", "INVALID\n"),
            ("one last trick\n430000000000000\n", "INVALID\n"),
        ];

        for (input, expected) in test_pairs.iter() {
            let mut input: &[u8] = input.as_bytes();
            let mut output: Vec<u8> = Vec::new();
            let ccn = get_ccn(&mut input, &mut output)
                .expect("stdin/stdout should work while getting ccn");
            let mut card = CardType::from_ccn(ccn);
            card.checksum();
            assert_eq!(card.to_string().as_bytes(), expected.as_bytes());
        }
    }
}
