use std::fs;
use std::io;
use anyhow::Result;
#[derive(Clone)] 
struct Person {
    nume: String,
    phone: String,
    age: i32,
}

fn main() -> Result<()> {
    let s = fs::read_to_string("src/file.txt")?;
    let mut youngest: Option<Person> = None;
    let mut oldest: Option<Person> = None;

    for line in s.lines() {
        let mut parts = line.split(',');
        if let (Some(name), Some(phone), Some(age)) = (parts.next(), parts.next(), parts.next()) {
            if let Ok(age) = age.trim().parse::<i32>() {
                let person = Person {
                    nume: name.to_string(),
                    phone: phone.to_string(),
                    age,
                };

                oldest = match oldest {
                    Some(ref o) if person.age > o.age => Some(person.clone()),
                    None => Some(person.clone()),
                    _ => oldest,
                };

                youngest = match youngest {
                    Some(ref y) if person.age < y.age => Some(person.clone()),
                    None => Some(person.clone()),
                    _ => youngest,
                };
            }
        }
    }

    if let Some(oldest) = oldest {
        println!("The oldest student is: {}, Phone: {}, Age: {}", oldest.nume, oldest.phone, oldest.age);
    }

    if let Some(youngest) = youngest {
        println!("The youngest student is: {}, Phone: {}, Age: {}", youngest.nume, youngest.phone, youngest.age);
    }

    Ok(())
}
