#[derive(Debug)]
enum MyError {
    Overflow,
}
fn check_add(n1: u32, n2: u32) -> Result<u32, MyError> {
    if n1 as u64 + n2 as u64 > u32::MAX as u64 {
        Err(MyError::Overflow)
    } else {
        let sum: u32 = n1 + n2;
        Ok(sum)
    }
}
fn check_multiply(n1: u32, n2: u32) -> Result<u32, MyError> {
    if n1 as u64 * n2 as u64 > u32::MAX as u64 {
        Err(MyError::Overflow)
    } else {
        let mult: u32 = n1 * n2;
        Ok(mult)
    }
}

fn main() {
    match check_add(u32::MAX, 1 as u32) {
        Ok(value) => {
            println!("{}\n", value);
        }
        Err(error) => {
            println!("{:?}\n", error);
        }
    }

    match check_multiply(1 as u32, 2 as u32) {
        Ok(value) => {
            println!("{}\n", value);
        }
        Err(error) => {
            println!("{:?}\n", error);
        }
    }
}
