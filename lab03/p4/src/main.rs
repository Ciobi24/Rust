#[derive(PartialEq, Debug)]
enum MyError {
    notASCII,
    notDIGIT,
    notBASE16DIGIT,
    notLETTER,
    notPRINTABLE,
}
fn to_uppercase(c: char) -> Result<char, MyError> {
    if c.is_alphabetic() {
        return Ok(c.to_ascii_uppercase());
    } else {
        return Err(MyError::notLETTER);
    }
}
fn to_lowercase(c: char) -> Result<char, MyError> {
    if c.is_alphabetic() {
        return Ok(c.to_ascii_lowercase());
    } else {
        return Err(MyError::notLETTER);
    }
}
fn print_char(c: char) -> Result<char, MyError> {
    if c.is_ascii_graphic() {
        return Ok(c);
    } else {
        return Err(MyError::notPRINTABLE);
    }
}
fn char_to_number(c: char) -> Result<u32, MyError> {
    if !c.is_ascii() {
        return Err(MyError::notASCII);
    }
    if !c.is_digit(10) {
        return Err(MyError::notDIGIT);
    }
    match c.to_digit(10) {
        Some(n) => return Ok(n),
        None => panic!("failed to_digit"),
    }
}
fn char_to_number_hex(c: char) -> Result<u32, MyError> {
    if !c.is_ascii() {
        return Err(MyError::notASCII);
    }
    if !c.is_digit(16) {
        return Err(MyError::notBASE16DIGIT);
    }
    match c.to_digit(16) {
        Some(n) => return Ok(n),
        None => panic!("failed to_digit_hex"),
    }
}
fn print_error(error: MyError) {
    match error {
        MyError::notASCII => println!("Your input is not ASCII\n"),
        MyError::notDIGIT => println!("Your character is not a digit\n"),
        MyError::notBASE16DIGIT => println!("Your character is not a base 16 digit\n"),
        MyError::notLETTER => println!("Your character is not a letter\n"),
        MyError::notPRINTABLE => println!("Your input is not printable\n"),
    }
}
fn main() {
    let c = '.';
    match to_uppercase(c) {
        Ok(up) => println!("Uppercase: {}\n", up),
        Err(error) => print_error(error),
        //Err(error)=>println!("Error:{:?}",error),
    }
    match to_lowercase(c) {
        Ok(low) => println!("Lowercase: {}\n", low),
        Err(error) => print_error(error),
        //Err(error)=>println!("Error:{:?}",error),
    }
    match print_char(c) {
        Ok(p) => println!("{} is printable\n", p),
        Err(error) => print_error(error),
        //Err(error)=>println!("Error:{:?}",error),
    }
    match char_to_number(c) {
        Ok(n) => println!("to_number:{}\n", n),
        Err(error) => print_error(error),
        //Err(error)=>println!("Error:{:?}",error),
    }
    match char_to_number_hex(c) {
        Ok(n) => println!("to_number_hex:{}\n", n),
        //Err(error)=>println!("Error:{:?}",error),
        Err(error) => print_error(error),
    }
}
