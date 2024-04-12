// first draft of a kind of rust rewrite of the cs50 library

use std::{
    io::{self, BufRead, Write},
    str::FromStr,
};

fn get<T: FromStr>(prompt: &str) -> io::Result<T> {
    let mut input = io::stdin().lock();
    let mut output = io::stdout();
    let result: T;
    loop {
        // prompt for user input
        output.write_all(prompt.as_bytes())?;
        output.flush()?;
        let mut user_input = String::new();
        input.read_line(&mut user_input)?;

        // verify that user input parses to type T
        result = match user_input.trim().parse() {
            Ok(parsed) => parsed,
            Err(_) => continue,
        };

        break;
    }
    Ok(result)
}

pub fn get_string(prompt: &str) -> io::Result<String> {
    get(prompt)
}

pub fn get_i8(prompt: &str) -> io::Result<i8> {
    get(prompt)
}

pub fn get_i16(prompt: &str) -> io::Result<i16> {
    get(prompt)
}

pub fn get_i32(prompt: &str) -> io::Result<i32> {
    get(prompt)
}

pub fn get_i64(prompt: &str) -> io::Result<i64> {
    get(prompt)
}

pub fn get_i128(prompt: &str) -> io::Result<i128> {
    get(prompt)
}

pub fn get_isize(prompt: &str) -> io::Result<isize> {
    get(prompt)
}

pub fn get_u8(prompt: &str) -> io::Result<u8> {
    get(prompt)
}

pub fn get_u16(prompt: &str) -> io::Result<u16> {
    get(prompt)
}

pub fn get_u32(prompt: &str) -> io::Result<u32> {
    get(prompt)
}

pub fn get_u64(prompt: &str) -> io::Result<u64> {
    get(prompt)
}

pub fn get_u128(prompt: &str) -> io::Result<u128> {
    get(prompt)
}

pub fn get_usize(prompt: &str) -> io::Result<usize> {
    get(prompt)
}

pub fn get_f32(prompt: &str) -> io::Result<f32> {
    get(prompt)
}

pub fn get_f64(prompt: &str) -> io::Result<f64> {
    get(prompt)
}

pub fn get_char(prompt: &str) -> io::Result<char> {
    get(prompt)
}

pub fn get_bool(prompt: &str) -> io::Result<bool> {
    get(prompt)
}
