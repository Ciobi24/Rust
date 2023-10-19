fn add_space(s: &mut String, n: i32) {
    for _i in 1..n {
        s.push(' ');
    }
}
fn add_str(s: &mut String, s1: &str) {
    *s += s1; //sau .as_str() din String
}
fn add_integer(s: &mut String, mut n: i32) {
    let mut n1: i32 = 0;
    if n < 0 {
        add_str(s, "-");
        n = 0 - n;
    }
    if n==0
    {
        s.push('0');
    }
    while n != 0 {
        n1 = n1 * 10 + n % 10;
        n /= 10;
    }
    let mut c: char;
    let mut i: i32 = 0;
    while n1 != 0 {
        i += 1;
        c = ((n1 % 10) as u8 + b'0') as char;
        if (i - 1) % 3 == 0&&(i-1)!=0 {
            s.push('_');
        }
        s.push(c);
        n1 /= 10;
    }
}
//o sa pun precizia ca parametru ca sa fie cat mai generala functia
fn add_float(s: &mut String, f: f32,mut p:i32) {
    let mut n: f32 = f.floor();
    if f < 0 as f32 {
        n += 1 as f32;
    }
    let ni: i32 = n as i32;
    add_integer(s, ni);
    s.push('.');
    //exista functia f32.fract()
    n = f - (ni as f32);
    //adauga zecimale random pt ca nu are precizie
    // while n > 0.0 {
    //     n *= 10.0;
    //     let digit = n.floor() as i32;
    //     s.push((digit as u8 + b'0') as char);
    //     n -= digit as f32;
    // }
    
    while p!=0{
        n*=10 as f32;
        p-=1;
        add_integer(s, n.floor()as i32);
        n=n-n.floor();
    }
}

fn main() {
    let s = &mut String::from("");
    add_space(s,40);
    add_str(s,"I");
    add_space(s,1);
    add_str(s,"💚");
    add_space(s,1);
    add_str(s,"\n");
    add_space(s,40);
    add_str(s,"RUST.");
    add_space(s,1);
    add_str(s,"\n");
    add_space(s,4);
    add_str(s,"\n");
    add_str(s,"Most");
    add_space(s,12);
    add_str(s,"crate");
    add_space(s,6);
    add_integer(s,306437968);
    add_space(s,11);
    add_str(s,"add");
    add_space(s,5);
    add_str(s,"lastest");
    add_space(s,9);
    add_str(s,"is");
    add_space(s,1);
    add_str(s,"\n");
    add_space(s,9);
    add_str(s,"downloaded");
    add_space(s,8);
    add_str(s,"has");
    add_space(s,13);
    add_str(s,"downloads");
    add_space(s,5);
    add_str(s,"the");
    add_space(s,9);
    add_str(s,"version");
    add_space(s,4);
    add_float(s,2.038,3);
    add_str(s,".");
    add_space(s,1);
    add_str(s,"\n");
    add_space(s,20);
    println!("{}",s);
}
