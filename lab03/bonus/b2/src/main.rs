use thiserror::Error;
#[derive(Error, PartialEq, Debug)]
enum MyError {
    #[error("{0} is not ascii")]
    notASCII(char),
    #[error("{0} is not a digit")]
    notDIGIT(char),
    #[error("{0} is not a base 16 digit")]
    notBASE16DIGIT(char),
    #[error("{0} is not a letter")]
    notLETTER(char),
    #[error("{0} is not printable")]
    notPRINTABLE(char),
}
fn to_uppercase(c: char) -> Result<char, MyError> {
    if c.is_alphabetic() {
        return Ok(c.to_ascii_uppercase());
    } else {
        return Err(MyError::notLETTER(c));
    }
}
fn to_lowercase(c: char) -> Result<char, MyError> {
    if c.is_alphabetic() {
        return Ok(c.to_ascii_lowercase());
    } else {
        return Err(MyError::notLETTER(c));
    }
}
fn print_char(c: char) -> Result<char, MyError> {
    if c.is_ascii_graphic() {
        return Ok(c);
    } else {
        return Err(MyError::notPRINTABLE(c));
    }
}
fn char_to_number(c: char) -> Result<u32, MyError> {
    if !c.is_ascii() {
        return Err(MyError::notASCII(c));
    }
    if !c.is_digit(10) {
        return Err(MyError::notDIGIT(c));
    }
    match c.to_digit(10) {
        Some(n) => return Ok(n),
        None => panic!("failed to_digit"),
    }
}
fn char_to_number_hex(c: char) -> Result<u32, MyError> {
    if !c.is_ascii() {
        return Err(MyError::notASCII(c));
    }
    if !c.is_digit(16) {
        return Err(MyError::notBASE16DIGIT(c));
    }
    match c.to_digit(16) {
        Some(n) => return Ok(n),
        None => panic!("failed to_digit_hex"),
    }
}

fn main() {
    let c = '.';
    match to_uppercase(c) {
        Ok(up) => println!("Uppercase: {}\n", up),
        Err(error) => println!("Error:{}", error),
    }
    match to_lowercase(c) {
        Ok(low) => println!("Lowercase: {}\n", low),
        Err(error) => println!("Error:{}", error),
    }
    match print_char(c) {
        Ok(p) => println!("{} is printable\n", p),
        Err(error) => println!("Error:{}", error),
    }
    match char_to_number(c) {
        Ok(n) => println!("to_number:{}\n", n),
        Err(error) => println!("Error:{}", error),
    }
    match char_to_number_hex(c) {
        Ok(n) => println!("to_number_hex:{}\n", n),
        Err(error) => println!("Error:{}", error),
    }
}
