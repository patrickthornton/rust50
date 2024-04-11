use std::io::{self, Write};

const DECLAMATION: &[u8] = b"hello, world\n";

fn main() {
    let mut output = io::stdout();
    declaim(&mut output).expect("writing a line to stdout should work");
}

fn declaim(output: &mut impl Write) -> io::Result<usize> {
    output.write(DECLAMATION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn what_did_you_say() {
        let mut output: Vec<u8> = Vec::new();
        let bytes_read = declaim(&mut output).expect("writing a line to stdout should work");

        assert_eq!(output, DECLAMATION);
        assert_eq!(bytes_read, DECLAMATION.len());
    }
}
