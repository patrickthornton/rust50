use std::io::{self, BufRead, Write};

const MAX_HEIGHT: u128 = 100;

fn main() {
    // BufRead needed for .read_line(); .lock() provides BufRead
    let mut input = io::stdin().lock();
    let mut output = io::stdout();
    let height =
        get_height(&mut input, &mut output).expect("stdin/stdout should work while getting height");
    pyramid(&mut output, height).expect("stdout should work while printing pyramid");
}

fn get_height(input: &mut impl BufRead, output: &mut impl Write) -> io::Result<u128> {
    let mut height: u128;
    loop {
        // prompt
        output.write_all(b"Height: ")?;
        output.flush()?;
        let mut height_input = String::new();
        input.read_line(&mut height_input)?;

        // verify that height is a number between 0 and MAX_HEIGHT
        height = match height_input.trim().parse() {
            Ok(height) => height,
            Err(_) => continue,
        };
        if height > MAX_HEIGHT {
            continue;
        }

        break;
    }
    Ok(height)
}

fn pyramid(output: &mut impl Write, height: u128) -> io::Result<()> {
    let mut buffer = String::new();
    for i in 1..=height {
        for _ in 0..height - i {
            buffer.push(' ');
        }
        for _ in 0..i {
            buffer.push('#');
        }
        buffer.push('\n');
    }
    output.write_all(buffer.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_height() {
        let mut input: &[u8] = b"-6\n9999999\nblah\n6\n";
        let mut output: Vec<u8> = Vec::new();
        let height = get_height(&mut input, &mut output)
            .expect("stdin/stdout should work while getting height");

        let expected = b"Height: Height: Height: Height: ";
        assert_eq!(output, expected);
        assert_eq!(height, 6);
    }

    #[test]
    fn test_pyramid_1() {
        let height = 1;
        let mut output: Vec<u8> = Vec::new();
        pyramid(&mut output, height).expect("stdin/stdout should work while printing pyramid");

        let expected = b"#\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_pyramid_2() {
        let height = 6;
        let mut output: Vec<u8> = Vec::new();
        pyramid(&mut output, height).expect("stdout should work while printing pyramid");

        let expected = b"     #\n    ##\n   ###\n  ####\n #####\n######\n";
        assert_eq!(output, expected);
    }
}
