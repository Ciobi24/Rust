fn prime(x: i32) -> bool {
    if x == 1 || x == 0 {
        return false;
    }
    if x == 2 {
        return true;
    }
    let mut d = 2;
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d += 1;
    }
    return true;
}

fn main() {
    print!("Primes between 0 and 100: \n");
    for i in 0..100 {
        if prime(i) == true {
            print!("{} ", i);
        }
    }
}
