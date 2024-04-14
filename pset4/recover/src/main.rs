// recovers jpegs from a .raw file
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};

// jpeg constants
const SOI: [u8; 2] = [0xff, 0xd8];
const APPN_1: u8 = 0xff;
const APPN_2: u8 = 0xe0; // to be used after &ing with 0xf0

fn is_jpeg(buffer: &[u8]) -> bool {
    buffer[0..=1] == SOI && buffer[2] == APPN_1 && buffer[3] & 0xf0 == APPN_2
}

const BLOCK_SIZE: usize = 512;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} file", args[0]);
        return;
    }

    let mut input = File::open(&args[1]).expect("should be able to open input file");

    recover(&mut input).expect("input and output should work when recovering jpegs from .raw file");
}

fn recover(input: &mut impl Read) -> Result<()> {
    // this use case (many small reads and writes) is perfect for buffered i/o
    let mut reader = BufReader::new(input);

    // recover jpegs
    let mut current_jpeg = 0;
    let buffer = &mut [0; BLOCK_SIZE];
    let mut jpeg_writer: Option<BufWriter<File>> = None;
    while let Ok(()) = reader.read_exact(buffer) {
        if is_jpeg(&buffer[0..4]) {
            // flush old writer if needed
            if let Some(mut writer) = jpeg_writer {
                writer.flush()?;
            }

            // create new writer
            let filename = format!("{:0>3}.jpg", current_jpeg);
            current_jpeg += 1;
            let file = File::create(filename).expect("should be able to create output file");
            jpeg_writer = Some(BufWriter::new(file));
        }

        if let Some(ref mut writer) = jpeg_writer {
            writer.write_all(buffer)?;
        }
    }
    Ok(())
}

// skipping tests like the awful person i am.
// but still, run it - out come 50 sensible-looking jpgs!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_jpeg() {
        let buffer = [0xff, 0xd8, 0xff, 0xe9];
        assert!(is_jpeg(&buffer));
        let bugger = [0xff, 0xd8, 0xff, 0x00];
        assert!(!is_jpeg(&bugger));
    }
}
