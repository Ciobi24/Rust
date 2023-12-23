fn check_add(n1: u32, n2: u32) -> u32 {
    if n1 as u64 + n2 as u64 > u32::MAX as u64 {
        panic!("{n1} + {n2} doesn't fit u32");
    } else {
        let sum: u32 = n1 + n2;
        return sum;
    }
}
fn check_multiply(n1: u32, n2: u32) -> u32 {
    if n1 as u64 * n2 as u64 > u32::MAX as u64 {
        panic!("{n1} * {n2} doesn't fit u32");
    } else {
        let mult: u32 = n1 * n2;
        return mult;
    }
}

fn main() {
    let mut rez: u32 = check_add(u32::MAX, 1 as u32);
    println!("{rez}\n");
    rez = check_multiply(1 as u32, 2 as u32);
    println!("{rez}\n")
}
