#[derive(Debug)]

enum MyError {
    NUMAR_NEGATIV,
}
fn factorial(n: u16) -> Option<u16> {
    let mut i = 1 as u16;
    let mut fact = 1 as u16;
    while fact as u32 * (i as u32 + 1 as u32) < u16::MAX as u32 && i + 1 <= n {
        fact = fact * (i + 1 as u16);
        i += 1;
    }
    if i == n {
        return Some(fact);
    }
    return None;
}
fn radical_parte_intreaga(n: i32) -> Result<i32, MyError> {
    if n < 0 as i32 {
        return Err(MyError::NUMAR_NEGATIV);
    }
    let mut i = 1 as i32;
    while i * i <= n {
        i += 1;
    }
    return Ok(i - 1);
}

fn main() {
    let n: u16 = 16;
    let n1: i32 = -1;
    match factorial(n) {
        Some(fact) => println!("factorial: {}\n", fact),
        None => println!("Factorialul lui {} depaseste u16\n", n),
    }
    match radical_parte_intreaga(n1) {
        Ok(r) => println!("Radical din {} este {}\n", n, r),
        Err(error) => println!("Eroare la calculul radicalului: {:?}", error),
    }
}
