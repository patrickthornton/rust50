// a simple program to modify the volume of a .wav file;
// we presume, as cs50 does, that the file uses only 16-bit samples
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: {} input.wav output.wav factor", args[0]);
        return;
    }

    let mut input = match File::open(&args[1]) {
        Ok(f) => f,
        Err(_) => {
            println!("Could not open input file: {}", args[1]);
            return;
        }
    };
    let mut output = match File::create(&args[2]) {
        Ok(f) => f,
        Err(_) => {
            println!("Could not open output file: {}", args[2]);
            return;
        }
    };
    let factor: f64 = match args[3].parse() {
        Ok(f) => f,
        Err(_) => {
            println!("Could not parse factor: {}", args[3]);
            return;
        }
    };

    volume(&mut input, &mut output, factor)
        .expect("input and output should work when modifying .wav file");
}

fn volume(input: &mut impl Read, output: &mut impl Write, factor: f64) -> Result<()> {
    // this use case (many small reads and writes) is perfect for buffered i/o
    let mut reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    // transfer 44-byte header
    let header: &mut [u8; 44] = &mut [0; 44];
    reader.read_exact(header)?;
    writer.write_all(header)?;

    // modify 16-bit samples
    let buffer: &mut [u8; 2] = &mut [0; 2];
    while let Ok(()) = reader.read_exact(buffer) {
        let mut sample = i16::from_le_bytes(*buffer) as f64;
        sample *= factor;
        *buffer = (sample as i16).to_le_bytes();
        writer.write_all(buffer)?;
    }
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_you_hear_the_music() {
        let fake_wav_header: &[u8; 44] = b"Super Duper Fake Wav Header With 44 Bytes!!!";
        let fake_wav_data: &[u8; 4] = b"\x30\x50\x10\x24";
        let input: &mut [u8; 48] = &mut [0; 48];
        input
            .iter_mut()
            .zip(fake_wav_header.iter().chain(fake_wav_data))
            .for_each(|(a, b)| *a = *b);
        let mut input = input.as_ref();
        let mut output: Vec<u8> = Vec::new();
        let factor = 2.;

        // the file does work, for what it's worth
        todo!("finish this test!");
    }
}
