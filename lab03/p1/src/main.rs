fn prime(x: u16) -> bool {
    if x == 1 || x == 0 {
        return false;
    }
    if x == 2 {
        return true;
    }
    let mut d: u32 = 2;
    while d * d <= x as u32 {
        if x as u32 % d == 0 {
            return false;
        }
        d += 1;
    }
    return true;
}
fn next_prime(x: u16) -> Option<u16> {
    let mut next = x + 1;
    while next < u16::MAX {
        if prime(next) == true {
            return Some(next);
        }
        next += 1;
    }
    return None;
}
fn main() {
    let mut n: u16 = 65000;
    while let Some(prime) = next_prime(n) {
        println!("The next prime after {} is: {}", n, prime);
        n = prime;
    }
    println!("No more prime numbers that fit in u16.");
}
