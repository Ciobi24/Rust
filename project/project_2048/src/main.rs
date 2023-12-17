use anyhow::anyhow;
use std::fs;

fn reading(matrix: &mut [[i32; 4]; 4]) -> Result<(), anyhow::Error> {
    let input = fs::read_to_string("src/progres.txt")?;
    let mut line_iter = input.lines();
    let flag_str = line_iter.next().unwrap_or_default(); // If file is empty, use default empty string
    let flag: i32 = match flag_str.parse::<i32>() {
        Ok(value) => value,
        Err(_) => return Err(anyhow!("Error parsing flag")),
    };
    if flag == 1 {
        for i in 0..4 {
            if let Some(line) = line_iter.next() {
                let mut numbers = line.split_whitespace();
                for j in 0..4 {
                    if let Some(number_str) = numbers.next() {
                        if let Ok(number) = number_str.parse::<i32>() {
                            matrix[i][j] = number;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let mut matrix: [[i32; 4]; 4] = [[0; 4]; 4];
    if let Err(err) = reading(&mut matrix) {
        println!("Error: {:?}", err);
        return;
    }

    for i in 0..4 {
        for j in 0..4 {
            print!("{:?} ", matrix[i][j]);
        }
        println!();
    }
}
