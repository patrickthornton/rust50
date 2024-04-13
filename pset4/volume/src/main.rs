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

    let mut input = File::open(&args[1]).expect("should be able to open input file");
    let mut output = File::create(&args[2]).expect("should be able to create output file");
    let factor: f64 = args[3]
        .parse()
        .expect("should be able to parse factor as a float");

    volume(&mut input, &mut output, factor)
        .expect("input and output should work when creating new .wav file");
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
        let fake_header: &[u8; 44] = b"Super Duper Fake Wav Header With 44 Bytes!!!";
        let fake_sample: &[u8; 2] = &1234_i16.to_le_bytes();
        let input: &mut [u8; 46] = &mut [0; 46];
        input
            .iter_mut()
            .zip(fake_header.iter().chain(fake_sample))
            .for_each(|(a, b)| *a = *b);
        let mut input = input.as_ref();
        let mut output: Vec<u8> = Vec::new();
        let factor = 2.;

        volume(&mut input, &mut output, factor)
            .expect("input and output should work when modifying .wav file");

        let expected: Vec<u8> = fake_header
            .iter()
            .chain(&2468_i16.to_le_bytes())
            .copied()
            .collect();
        assert_eq!(output, expected);
    }

    #[test]
    fn can_you_turn_that_racket_off() {
        let fake_header: &[u8; 44] = b"Hyper Ultra Mega Wav Header With 44 Bytes???";
        let fake_sample_1: &[u8; 2] = &9876_i16.to_le_bytes();
        let fake_sample_2: &[u8; 2] = &(-2024_i16).to_le_bytes();
        let input: &mut [u8; 48] = &mut [0; 48];
        input
            .iter_mut()
            .zip(fake_header.iter().chain(fake_sample_1).chain(fake_sample_2))
            .for_each(|(a, b)| *a = *b);
        let mut input = input.as_ref();
        let mut output: Vec<u8> = Vec::new();
        let factor = 0.;

        volume(&mut input, &mut output, factor)
            .expect("input and output should work when modifying .wav file");

        let expected: Vec<u8> = fake_header
            .iter()
            .chain(&0_i16.to_le_bytes())
            .chain(&0_i16.to_le_bytes())
            .copied()
            .collect();
        assert_eq!(output, expected);
    }
}
