use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Read};
extern crate base64;
const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn base64_encode(data: &[u8]) -> String {
    let mut result = String::new();
    let mut index = 0;

    while index < data.len() {
        let first = data[index];
        let second = if index + 1 < data.len() { data[index + 1] } else { 0 };
        let third = if index + 2 < data.len() { data[index + 2] } else { 0 };

        result.push(BASE64_CHARS[(first >> 2) as usize] as char);
        result.push(BASE64_CHARS[((first & 0b11) << 4 | (second >> 4)) as usize] as char);

        if index + 1 < data.len() {
            result.push(BASE64_CHARS[((second & 0b1111) << 2 | (third >> 6)) as usize] as char);
        } else {
            result.push('=');
        }

        if index + 2 < data.len() {
            result.push(BASE64_CHARS[(third & 0b111111) as usize] as char);
        } else {
            result.push('=');
        }

        index += 3;
    }

    result
}

fn main() {
    println!("encoder, version 0.1.0, built for {}", if cfg!(target_os = "windows") { "Windows" } else { "Other OS" });
    let matches = App::new("Encoder")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Encode data to base64")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Sets the input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets the output file")
                .takes_value(true),
        )
        .get_matches();

        if let (Some(input_file), Some(output_file)) = (matches.value_of("input"), matches.value_of("output")) {
            let mut input = std::fs::read(input_file).expect("Failed to read input file");
            let encoded = base64::encode(&input);
            std::fs::write(output_file, encoded).expect("Failed to write to output file")}
             else {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");

        let encoded = base64::encode(input.trim().as_bytes());
        println!("{}", encoded);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encoding() {
        let test_cases = [
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("hello world", "aGVsbG8gd29ybGQ="),
        ];

        for (input, expected_output) in &test_cases {
            let encoded = base64_encode(input.as_bytes());
            assert_eq!(encoded, *expected_output);
        }
    }
}