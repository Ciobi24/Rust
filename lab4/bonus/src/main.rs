use std::fs;
use std::io;
use std::io::Write;
use std::time::Instant;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error), 
    #[error("Error: character is not ASCII")]
    NotASCII,
}

fn create_file() -> Result<(), MyError> {
    let text_to_repeat = "This is the text to repeat ";
    let file_size: usize = 4 * 1024 * 1024 * 1024; // 4GiB file size

    let mut file = fs::File::create("large_file.txt")?;

    while file.metadata()?.len() < file_size as u64 {
        file.write(text_to_repeat.as_bytes())?;
    }

    Ok(())
}

fn rotate(c: char, base: u8) -> char {
    let rotated_char = (c as u8 - base + 13) % 26 + base;
    rotated_char as char
}

fn p2() -> Result<(), MyError> {
    let s = fs::read_to_string("large_file.txt")?;
    let mut new_s = String::new();
    for c in s.chars() {
        match c {
            'a'..='z' => new_s.push(rotate(c, b'a')),
            'A'..='Z' => new_s.push(rotate(c, b'A')),
            '\n' => new_s.push('\n'),
            ' ' => new_s.push(' '),
            _ => return Err(MyError::NotASCII),
        }
    }
    println!("old string {} \n new string {}", s, new_s);
    Ok(())
}

fn do_stuff() {
    create_file();
    p2().expect("An error occurred");
}

fn main() {
    let start = Instant::now();
    do_stuff();
    println!("{:?}", start.elapsed());
}

// use std::fs;
// use std::io;
// use std::time::Instant;
// use thiserror::Error;
// use std::io::BufRead;
// use std::io::Write;

// #[derive(Error, Debug)]
// enum MyError {
//     #[error("IO error: {0}")]
//     IoError(#[from] io::Error),
//     #[error("Error: character is not ASCII")]
//     NotASCII,
// }

// fn create_file() -> Result<(), MyError> {
//     let text_to_repeat = "This is the text to repeat ";
//     let file_size: usize = 4 * 1024 * 1024 * 1024; // 4GiB file size

//     let mut file = fs::File::create("large_file.txt")?;

//     while file.metadata()?.len() < file_size as u64 {
//         file.write(text_to_repeat.as_bytes())?;
//     }

//     Ok(())
// }

// fn rotate(c: char, base: u8) -> char {
//     let rotated_char = (c as u8 - base + 13) % 26 + base;
//     rotated_char as char
// }

// fn p2() -> Result<(), MyError> {
//     let file_path = "large_file.txt";
//     let file = fs::File::open(file_path)?;
//     let reader = io::BufReader::new(file);

//     let mut new_s = String::new();

//     for line_result in reader.lines() {
//         let line = line_result?;
//         for c in line.chars() {
//             match c {
//                 'a'..='z' => new_s.push(rotate(c, b'a')),
//                 'A'..='Z' => new_s.push(rotate(c, b'A')),
//                 '\n' => new_s.push('\n'),
//                 ' ' => new_s.push(' '),
//                 _ => return Err(MyError::NotASCII),
//             }
//         }
//     }

//     println!("Processed the file.");
//     Ok(())
// }

// fn do_stuff() {
//     create_file();
//     p2().expect("An error occurred");
// }

// fn main() {
//     let start = Instant::now();
//     do_stuff();
//     println!("{:?}", start.elapsed());
// }
