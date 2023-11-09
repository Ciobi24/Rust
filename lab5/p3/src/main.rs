use std::fs;
use std::io;
use anyhow::Result;
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct Person {
    name: String,
    phone: String,
    age: i32,
}

fn main() -> Result<()> {
    let s = fs::read_to_string("./src/person.txt")?;
    let mut youngest: Option<Person> = None;
    let mut oldest: Option<Person> = None;

    for line in s.lines() {
        let person: Person = serde_json::from_str(line)?;

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

    if let Some(oldest) = oldest {
        println!("The oldest student is: {}, Phone: {}, Age: {}", oldest.name, oldest.phone, oldest.age);
    }

    if let Some(youngest) = youngest {
        println!("The youngest student is: {}, Phone: {}, Age: {}", youngest.name, youngest.phone, youngest.age);
    }

    Ok(())
}
