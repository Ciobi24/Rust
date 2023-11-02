use std::{fs, io};

fn p3() -> Result<(), io::Error> {
    let abv = "pentru pt\npentru ptr\ndomnul dl\ndoamna dna";
    let s = fs::read_to_string("src/file.txt")?;
    let mut new_s = String::new();
    let mut cuv = String::new();

    for c in s.chars() {
        if c == ' ' {
            if !cuv.is_empty() {
                let mut expanded = cuv.to_string();
                for line in abv.lines() {
                    if line.contains(&cuv) {
                        expanded = line.split(' ').next().unwrap_or(&cuv).to_string();
                        break;
                    }
                }
                new_s.push_str(&expanded);
                cuv.clear();
            }
            new_s.push(' ');
        } else {
            cuv.push(c);
        }
    }

    if !cuv.is_empty() {
        let mut expanded = cuv.to_string();
        for line in abv.lines() {
            if line.contains(&cuv) {
                expanded = line.split(' ').next().unwrap_or(&cuv).to_string();
                break;
            }
        }
        new_s.push_str(&expanded);
    }

    println!("old string {}\nnew string {}", s, new_s);
    Ok(())
}

fn main() {
    p3().expect("Error");
}
