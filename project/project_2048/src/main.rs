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
fn up(matrix: &mut [[i32; 4]; 4]) {
    for j in 0..4 {
        let mut zero_count = 0;
        for i in 0..4 {
            if matrix[i][j] == 0 {
                zero_count += 1;
            } else if zero_count > 0 {
                matrix[i - zero_count][j] = matrix[i][j];
                matrix[i][j] = 0;
            }
        }
    }
    for j in 0..4 {
        for i in 0..3 {
            if matrix[i][j] == matrix[i + 1][j] {
                matrix[i][j] = matrix[i][j] + matrix[i + 1][j];
                for k in (i + 1)..3 {
                    matrix[k][j] = matrix[k + 1][j];
                }
                matrix[3][j] = 0 as i32;
            }
        }
    }
}

fn down(matrix: &mut [[i32; 4]; 4]) {
    for j in 0..4 {
        let mut zero_count = 0;
        let mut i=3;
        loop{
            if matrix[i][j] == 0 {
                zero_count += 1;
            } else if zero_count > 0 {
                matrix[i + zero_count][j] = matrix[i][j];
                matrix[i][j] = 0;
            }
            if i == 0 as usize{
                break;
            }
            i=i-1;
        }
    }
    for j in 0..4 {
        let mut i=3;
        loop {
            if matrix[i][j] == matrix[i - 1][j] {
                matrix[i][j] = matrix[i][j] + matrix[i - 1][j];
                let mut k=i-1;
                loop {
                    if k==0{
                        break;
                    }
                    matrix[k][j] = matrix[k - 1][j];
                    k=k-1;
                }
                matrix[0][j] = 0 as i32;
            }
            if i==1{
                break;
            }
            i=i-1;
        }
    }
}



fn left(matrix: &mut [[i32; 4]; 4]) {
    for i in 0..4 {
        let mut zero_count = 0;
        for j in 0..4 {
            if matrix[i][j] == 0 {
                zero_count += 1;
            } else if zero_count > 0 {
                matrix[i][j-zero_count] = matrix[i][j];
                matrix[i][j] = 0;
            }
        }
    }
    for i in 0..4 {
        for j in 0..3 {
            if matrix[i][j] == matrix[i][j+1] {
                matrix[i][j] = matrix[i][j] + matrix[i][j+1];
                for k in (j + 1)..3 {
                    matrix[i][k] = matrix[i][k+1];
                }
                matrix[i][3] = 0 as i32;
            }
        }
    }
}
fn right(matrix: &mut [[i32; 4]; 4]) {
    for i in 0..4 {
        let mut zero_count = 0;
        let mut j=3;
        loop {
            if matrix[i][j] == 0 {
                zero_count += 1;
            } else if zero_count > 0 {
                matrix[i][j+zero_count] = matrix[i][j];
                matrix[i][j] = 0;
            }
            if j==0{break;}
            j=j-1;
        }
    }
    for i in 0..4 {
        let mut j=3;
        loop {
            if j==0{
                break;
            }
            if matrix[i][j] == matrix[i][j-1] {
                matrix[i][j] = matrix[i][j] + matrix[i][j-1];
                let mut k=j-1;
                loop {
                    if k==0{
                        break;
                    }
                    matrix[i][k] = matrix[i][k-1];
                    k=k-1;
                }
                matrix[i][0] = 0 as i32;
            }
            j=j-1;
        }
    }
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
