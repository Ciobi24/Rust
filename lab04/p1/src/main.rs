use std::{fs, io};

fn p1() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/file.txt")?;
    let mut bytes: i32;
    let mut charact: i32;
    let mut maxbytes: i32 = 0;
    let mut maxcharact: i32 = 0;
    let mut maxline_bytes= String::from("\u{0}");
    let mut maxline_charact= String::from("\u{0}");
    for i in s.lines() {
        bytes = i.len() as i32;
        charact=0;
        for j in i.chars(){
            charact+=1;
        }
        if bytes > maxbytes {
            maxline_bytes = i.to_string();
            maxbytes = bytes;
        }
        if charact > maxcharact {
            maxline_charact = i.to_string();
            maxcharact = charact;
        }
    }
    println!("{}", maxline_bytes);
    println!("{}", maxline_charact);
    Ok(())
}

fn main() {
    p1();
}
