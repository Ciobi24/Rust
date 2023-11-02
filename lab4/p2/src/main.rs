use thiserror::Error;
use std::{fs, io};
#[derive(Error,Debug)]
enum MyError {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("Error: character is not ASCII")]
    NotASCII,
}
fn rotate(c: char, base: u8) -> char {
    let rotated_char = (c as u8 - base + 13) % 26 + base;
    rotated_char as char
}

fn p2() -> Result<(), MyError> {
    let s = fs::read_to_string("src/file.txt")?;
    let mut new_s = String::new();
    for c in s.chars() {
        match c {
            'a'..='z' => new_s.push(rotate(c, b'a')),
            'A'..='Z' => new_s.push(rotate(c, b'A')),
            '\n' => new_s.push('\n'),
            ' '=>new_s.push(' '),
            _ => return Err(MyError::NotASCII),
        }
    }
    println!("old string {} \n new string {}", s, new_s);
    Ok(())
}

fn main() {
    p2().expect("An error occurred");
}
