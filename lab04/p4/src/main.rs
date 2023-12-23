use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let file_path = "C:\\Windows\\System32\\drivers\\etc\\hosts";

    let hosts_file = fs::read_to_string(file_path)?;

    for line in hosts_file.lines() {

        if line.trim().starts_with("#") {
            continue;
        }

        let mut first_column = String::new();
        let mut second_column = String::new();
        let mut space_count = 0;

        for c in line.chars() {
            //cate col am procesat
            if c.is_whitespace() {
                space_count += 1;
                if space_count >= 2 {
                    break;
                }
            } else {
                //prima col
                if space_count == 0 {
                    first_column.push(c);
                    //a doua col
                } else if space_count == 1 {
                    second_column.push(c);
                }
            }
        }

        println!("{} => {}", first_column, second_column);
    }

    Ok(())
}
