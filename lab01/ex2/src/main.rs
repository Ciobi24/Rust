fn coprime(mut a: i32, mut b: i32) -> bool {
    if a == 0 || b == 0 {
        return false;
    }
    let mut r = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }
    if b == 1 {
        return true;
    } else {
        return false;
    }
}
fn main() {
    for i in 0..100 {
        for j in 0..100 {
           if coprime(i, j)==true{
            print!("{} si {} sunt coprime\n",i,j);
           }
        }
    }
}
