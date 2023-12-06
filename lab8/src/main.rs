use std::collections::HashMap;
use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let text = fs::read_to_string("src/text.txt")?;
    
    let mut words: Vec<&str> = Vec::new();
    for part in text.split(|c: char| !c.is_alphanumeric()) {
        for word in part.split_whitespace() {
            if !word.is_empty() {
                words.push(word);
            }
        }
    }
    
    let mut map: HashMap<String, i32> = HashMap::new();
    
    for i in words.iter() {
        map.entry(i.to_lowercase()).and_modify(|v| *v += 1).or_insert(1);
    }
    
    let mut to_sort:Vec<(String,i32)>=Vec::new();
    for (k,v) in &map{
        to_sort.push((k.clone(),*v));
    }
    to_sort.sort_by(|a, b| {
        if a.1 != b.1 {
            b.1.cmp(&a.1) // Sort by values in descending order
        } else {
            a.0.cmp(&b.0) // If values are equal, sort keys alphabetically
        }});

        let max_word_length = map
        .iter()
        .map(|(word, _)| word.len())
        .max()
        .unwrap_or(0);

    for (k,v) in &to_sort{
        println!("{:<width$} => {}",k,v,width=max_word_length);
    }
    
    Ok(())
}
