use std::io::{self, BufRead, Write};

fn main() {
    // BufRead needed for .read_line(); .lock() provides BufRead
    let mut input = io::stdin().lock();
    let mut output = io::stdout();
    converse(&mut input, &mut output).expect("stdin/stdout should work");
}

fn converse(input: &mut impl BufRead, output: &mut impl Write) -> io::Result<usize> {
    // prompt
    output.write(b"What's your name? ")?;
    output.flush()?;
    let mut name = String::new();
    input.read_line(&mut name)?;

    // answer
    let reply = "hello, ".to_owned() + &name + "\n";
    output.write(reply.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn come_again() {
        let mut input: &[u8] = b"patrick";
        let mut output: Vec<u8> = Vec::new();
        let bytes_written = converse(&mut input, &mut output).expect("stdin/stdout should work");

        let expected = b"What's your name? hello, patrick\n";
        let expected_reply = b"hello, patrick\n";
        assert_eq!(output, expected);
        assert_eq!(bytes_written, expected_reply.len());
    }
}
