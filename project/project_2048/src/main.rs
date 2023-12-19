use anyhow::anyhow;
use std::fs;
use rand::Rng;
use std::io;

fn reading(matrix: &mut [[i32; 4]; 4]) -> Result<(), anyhow::Error> {
    let input = fs::read_to_string("src/progres.txt")?;
    let mut line_iter = input.lines();
    let flag_str = line_iter.next().unwrap_or_default(); // If file is empty, use default empty string
    let flag: i32 = match flag_str.parse::<i32>() {
        Ok(value) => value,
        Err(_) => return Err(anyhow!("Error parsing flag")),
    };
    if flag == 1 {
        for row in matrix.iter_mut().take(4) {
            if let Some(line) = line_iter.next() {
                let mut numbers = line.split_whitespace();
                for elem in row.iter_mut().take(4) {
                    if let Some(number_str) = numbers.next() {
                        if let Ok(number) = number_str.parse::<i32>() {
                            *elem = number;
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
                matrix[i][j] += matrix[i + 1][j];
                for k in (i + 1)..3 {
                    matrix[k][j] = matrix[k + 1][j];
                }
                matrix[3][j] = 0_i32;
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
            if i == 0_usize{
                break;
            }
            i-=1;
        }
    }
    for j in 0..4 {
        let mut i=3;
        loop {
            if matrix[i][j] == matrix[i - 1][j] {
                matrix[i][j] += matrix[i - 1][j];
                let mut k=i-1;
                loop {
                    if k==0{
                        break;
                    }
                    matrix[k][j] = matrix[k - 1][j];
                    k-=1;
                }
                matrix[0][j] = 0_i32;
            }
            if i==1{
                break;
            }
            i-=1;
        }
    }
}



fn left(matrix: &mut [[i32; 4]; 4]) {
    for row in matrix.iter_mut().take(4) {
        let mut temp_row = [0; 4]; // Create a temporary row to store modified values

        let mut zero_count = 0;
        for (i, elem) in row.iter_mut().enumerate() {
            if *elem == 0 {
                zero_count += 1;
            } else {
                temp_row[i - zero_count] = *elem;
                *elem = 0;
            }
        }

        // Copy elements from the temporary row back to the original row
        for (i, &val) in temp_row.iter().enumerate() {
            row[i] = val;
        }
    }


    for row in matrix.iter_mut().take(4) {
        for j in 0..3 {
            if row[j] == row[j+1] {
                row[j] += row[j+1];
                for k in (j + 1)..3 {
                    row[k] = row[k+1];
                }
                row[3] = 0_i32;
            }
        }
    }
}
fn right(matrix: &mut [[i32; 4]; 4]) {
    for row in matrix.iter_mut().take(4) {
        let mut zero_count = 0;
        let mut j=3;
        loop {
            if row[j] == 0 {
                zero_count += 1;
            } else if zero_count > 0 {
                row[j+zero_count] = row[j];
                row[j] = 0;
            }
            if j==0{break;}
            j-=1;
        }
    }
    for row in matrix.iter_mut().take(4) {
        let mut j=3;
        loop {
            if j==0{
                break;
            }
            if row[j] == row[j-1] {
                row[j] += row[j-1];
                let mut k=j-1;
                loop {
                    if k==0{
                        break;
                    }
                    row[k] = row[k-1];
                    k-=1;
                }
                row[0] = 0_i32;
            }
            j-=1;
        }
    }
}
fn generate_random(matrix:&mut [[i32;4];4]){
    let mut rng = rand::thread_rng();
    let rand_num: f64 = rng.gen(); // Generate a random floating-point number between 0 and 1
    let mut empty_place=0;
    let mut list:Vec<i32>=Vec::new();
    for row in matrix.iter_mut().take(4) {
        for elem in row.iter_mut().take(4) {
            empty_place+=1;
            if *elem==0{
                list.push(empty_place);
            }
        }
    }
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..list.len()); // Generate a random index within the list's bounds
    let i=(list[index]-1)/4;
    let j=(list[index]-1)%4;
    if rand_num < 0.9 {
        // 90% chance for generating 2
        matrix[i as usize][j as usize]=2;
    } else {
        // 10% chance for generating 4
        matrix[i as usize][j as usize]=4;
    }
}
fn update_file(game_option:i32,matrix:&mut [[i32;4];4])->Result<(),anyhow::Error>{
if game_option==0{
    let mut content_to_write:String =String::from("0\n"); // Contents to write to the file
    let mut i=0;
    loop{
        content_to_write.push('0');
        i+=1;
        if i==16{break;}
        if i%4==0 {
            content_to_write.push('\n');
        }
        else{
            content_to_write.push(' ');
        }
    }
    fs::write("src/progres.txt", content_to_write)?;
}
else{
    let mut content_to_write = String::from("1\n");
    for row in matrix.iter() {
        for &elem in row.iter() {
            content_to_write.push_str(&elem.to_string());
            if content_to_write.matches(' ').count() % 4 == 3 {
                content_to_write.push('\n');
            }
            else{
            content_to_write.push(' ');
            }
        }
    }

    fs::write("src/progres.txt", content_to_write)?;

}
Ok(())
}
fn main() {
    let mut matrix: [[i32; 4]; 4] = [[0; 4]; 4];

    let mut input = String::new();
    let mut read=false;
    while !read{
        println!("Press the key for an option: \n n => new game \n c => continue\n");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if let Some(ch) = input.trim().chars().next() {
                read=true;
                if ch=='n' || ch=='N'{
                    match update_file(0, &mut matrix) {
                        Ok(_) => println!("File updated successfully"),
                        Err(err) => eprintln!("Error updating file: {}", err),
                    }
                }
                else if ch=='c' ||ch=='C' {
                    if let Err(err) = reading(&mut matrix) {
                        println!("Error: {:?}", err);
                        return;
                    }
                }
                else{
                    read=false;
                    println!("Wrong key");
                }
            } else {
                println!("No character entered");
            }
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
        }
    }
    }

    for row in matrix.iter_mut().take(4) {
        for elem in row.iter_mut().take(4) {
            print!("{:?} ", *elem);
        }
        println!();
    }
}
