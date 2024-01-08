use anyhow::anyhow;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use rand::Rng;
use std::fs;
use std::io;
use prettytable::{Table, Row, Cell};
use prettytable::format;

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
                        }else {
                            return Err(anyhow!("Error parsing number"));
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
        let mut i = 3;
        loop {
            if matrix[i][j] == 0 {
                zero_count += 1;
            } else if zero_count > 0 {
                matrix[i + zero_count][j] = matrix[i][j];
                matrix[i][j] = 0;
            }
            if i == 0_usize {
                break;
            }
            i -= 1;
        }
    }
    for j in 0..4 {
        let mut i = 3;
        loop {
            if matrix[i][j] == matrix[i - 1][j] {
                matrix[i][j] += matrix[i - 1][j];
                let mut k = i - 1;
                loop {
                    if k == 0 {
                        break;
                    }
                    matrix[k][j] = matrix[k - 1][j];
                    k -= 1;
                }
                matrix[0][j] = 0_i32;
            }
            if i == 1 {
                break;
            }
            i -= 1;
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
            if row[j] == row[j + 1] {
                row[j] += row[j + 1];
                for k in (j + 1)..3 {
                    row[k] = row[k + 1];
                }
                row[3] = 0_i32;
            }
        }
    }
}
fn right(matrix: &mut [[i32; 4]; 4]) {
    for row in matrix.iter_mut().take(4) {
        let mut zero_count = 0;
        let mut j = 3;
        loop {
            if row[j] == 0 {
                zero_count += 1;
            } else if zero_count > 0 {
                row[j + zero_count] = row[j];
                row[j] = 0;
            }
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
    for row in matrix.iter_mut().take(4) {
        let mut j = 3;
        loop {
            if j == 0 {
                break;
            }
            if row[j] == row[j - 1] {
                row[j] += row[j - 1];
                let mut k = j - 1;
                loop {
                    if k == 0 {
                        break;
                    }
                    row[k] = row[k - 1];
                    k -= 1;
                }
                row[0] = 0_i32;
            }
            j -= 1;
        }
    }
}
fn generate_random(matrix: &mut [[i32; 4]; 4]) {
    let mut rng = rand::thread_rng();
    let rand_num: f64 = rng.gen(); // Generate a random floating-point number between 0 and 1
    let mut empty_place = 0;
    let mut list: Vec<i32> = Vec::new();
    for row in matrix.iter_mut().take(4) {
        for elem in row.iter_mut().take(4) {
            empty_place += 1;
            if *elem == 0 {
                list.push(empty_place);
            }
        }
    }
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..list.len()); // Generate a random index within the list's bounds
    let i = (list[index] - 1) / 4;
    let j = (list[index] - 1) % 4;
    if rand_num < 0.9 {
        // 90% chance for generating 2
        matrix[i as usize][j as usize] = 2;
    } else {
        // 10% chance for generating 4
        matrix[i as usize][j as usize] = 4;
    }
}
fn update_file(game_option: i32, matrix: &mut [[i32; 4]; 4]) -> Result<(), anyhow::Error> {
    if game_option == 0 {
        let mut content_to_write: String = String::from("0\n"); // Contents to write to the file
        let mut i = 0;
        loop {
            content_to_write.push('0');
            i += 1;
            if i == 16 {
                break;
            }
            if i % 4 == 0 {
                content_to_write.push('\n');
            } else {
                content_to_write.push(' ');
            }
        }
        fs::write("src/progres.txt", content_to_write)?;
    } else {
        let mut content_to_write = String::from("1\n");
        let mut i = 0;
        for row in matrix.iter() {
            for &elem in row.iter() {
                content_to_write.push_str(&elem.to_string());
                i += 1;
                if i % 4 == 0 {
                    content_to_write.push('\n');
                } else {
                    content_to_write.push(' ');
                }
            }
        }

        fs::write("src/progres.txt", content_to_write)?;
    }
    Ok(())
}
fn game_start(matrix: &mut [[i32; 4]; 4]) {
    let mut input = String::new();
    let mut read = false;
    while !read {
        println!("Press the key for an option: \n n => new game \n c => continue\n");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Some(ch) = input.trim().chars().next() {
                    read = true;
                    if ch == 'n' || ch == 'N' {
                        generate_random(matrix);
                        generate_random(matrix);
                        match update_file(1, matrix) {
                            Ok(_) => {}
                            Err(err) => {
                                eprintln!("Error updating file: {}", err);
                                return;
                            }
                        }
                    } else if ch == 'c' || ch == 'C' {
                        if let Err(err) = reading(matrix) {
                            println!("Error: {:?}", err);
                            return;
                        }
                    } else {
                        read = false;
                        println!("Wrong key");
                    }
                } else {
                    println!("No character entered");
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                return;
            }
        }
    }
}
fn print_instructions() {
    println!("Use the arrows, one at a time. Press esc to exit.\nTry to sum the numbers up to 2048.\n   Good luck!");
}


fn print_matrix(matrix: &mut [[i32; 4]; 4]) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    for row in matrix.iter_mut().take(4) {
        let mut row_data = Vec::new();
        for elem in row.iter_mut().take(4) {
            let color_code = match *elem {
                2 => "\x1b[31m",     // Red
                4 => "\x1b[32m",     // Green
                8 => "\x1b[33m",     // Yellow
                16 => "\x1b[34m",    // Blue
                32 => "\x1b[35m",    // Magenta
                64 => "\x1b[36m",    // Cyan
                128 => "\x1b[95m",   // Bright Magenta
                256 => "\x1b[91m",   // Bright Red
                512 => "\x1b[92m",   // Bright Green
                1024 => "\x1b[93m",  // Bright Yellow
                2048 => "\x1b[94m",  // Bright Blue
                _ => "\x1b[0m",      // Reset
            };
            row_data.push(Cell::new(&format!("{} {:?} \x1b[0m", color_code, *elem)));
        }
        table.add_row(Row::new(row_data));
    }
    table.printstd();
}

fn clear_screen() {
    execute!(std::io::stdout(), Clear(ClearType::All)).expect("Failed to clear screen");
}
fn check(matrix: &[[i32; 4]; 4]) -> bool {
    let mut copy = [[0; 4]; 4]; 
    
    for i in 0..4 {
        for j in 0..4 {
            copy[i][j] = matrix[i][j];
        }
    }

    up(&mut copy);
    down(&mut copy);
    left(&mut copy);
    right(&mut copy);


    for i in 0..4 {
        for j in 0..4 {
            if matrix[i][j] != copy[i][j] {
                return true;
            }
        }
    }
    false
}



fn game_logic(matrix: &mut [[i32; 4]; 4]) {
    loop {
        print_matrix(matrix);
        let input = read();
        match input {
            Ok(event) => {
                if let crossterm::event::Event::Key(KeyEvent {
                    code, modifiers, ..
                }) = event
                {
                    if modifiers == KeyModifiers::NONE {
                        match code {
                            KeyCode::Up => {
                                println!("Up arrow pressed");
                                clear_screen();
                                up(matrix);
                                let mut space = false;
                                let mut win = false;
                                for row in matrix.iter_mut().take(4) {
                                    for elem in row.iter_mut().take(4) {
                                        if *elem == 0 {
                                            space = true;
                                        }
                                        if *elem == 2048 {
                                            win = true;
                                        }
                                    }
                                }
                                if win {
                                    clear_screen();
                                    print_matrix(matrix);
                                    println!("\n     ----YOU WON! GOOD JOB!----");
                                    match update_file(0, matrix) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln!("Error updating file: {}", err);
                                            return;
                                        }
                                    }
                                    break;
                                }
                                if space {
                                    generate_random(matrix);
                                    match update_file(1, matrix) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln!("Error updating file: {}", err);
                                            return;
                                        }
                                    }
                                } else if !check(matrix)
                                 {
                                    println!("\n    ----GAME OVER----");
                                    *matrix = [[0; 4]; 4];
                                    generate_random(matrix);
                                    generate_random(matrix);
                                    match update_file(1, matrix) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln!("Error updating file: {}", err);
                                            return;
                                        }
                                    }
                                    break;
                                }
                            }
                            KeyCode::Down => {
                                println!("Down arrow pressed");
                                clear_screen();
                                down(matrix);
                                let mut space = false;
                                let mut win = false;
                                for row in matrix.iter_mut().take(4) {
                                    for elem in row.iter_mut().take(4) {
                                        if *elem == 0 {
                                            space = true;
                                        }
                                        if *elem == 2048 {
                                            win = true;
                                        }
                                    }
                                }
                                if win {
                                    clear_screen();
                                    print_matrix(matrix);
                                    println!("\n     ----YOU WON! GOOD JOB!----");
                                    match update_file(0, matrix) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln!("Error updating file: {}", err);
                                            return;
                                        }
                                    }
                                    break;
                                }
                                if space {
                                    generate_random(matrix);
                                    match update_file(1, matrix) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln!("Error updating file: {}", err);
                                            return;
                                        }
                                    }
                                } else if !check(matrix){
                                    println!("\n    ----GAME OVER----");
                                    *matrix = [[0; 4]; 4];
                                    generate_random(matrix);
                                    generate_random(matrix);
                                    match update_file(1, matrix) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln!("Error updating file: {}", err);
                                            return;
                                        }
                                    }

                                    break;
                                }
                            }
                            KeyCode::Left => {
                                println!("Left arrow pressed");
                                clear_screen();
                                left(matrix);
                                let mut space = false;
                                let mut win = false;
                                for row in matrix.iter_mut().take(4) {
                                    for elem in row.iter_mut().take(4) {
                                        if *elem == 0 {
                                            space = true;
                                        }
                                        if *elem == 2048 {
                                            win = true;
                                        }
                                    }
                                }
                                if win {
                                    clear_screen();
                                    print_matrix(matrix);
                                    println!("\n     ----YOU WON! GOOD JOB!----");
                                    match update_file(0, matrix) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln!("Error updating file: {}", err);
                                            return;
                                        }
                                    }
                                    break;
                                }
                                if space {
                                    generate_random(matrix);
                                    match update_file(1, matrix) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln!("Error updating file: {}", err);
                                            return;
                                        }
                                    }
                                } else if !check(matrix) {
                                    println!("\n    ----GAME OVER----");
                                    *matrix = [[0; 4]; 4];
                                    generate_random(matrix);
                                    generate_random(matrix);
                                    match update_file(1, matrix) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln!("Error updating file: {}", err);
                                            return;
                                        }
                                    }
                                    break;
                                }
                            }
                            KeyCode::Right => {
                                println!("Right arrow pressed");
                                clear_screen();
                                right(matrix);
                                let mut space = false;
                                let mut win = false;
                                for row in matrix.iter_mut().take(4) {
                                    for elem in row.iter_mut().take(4) {
                                        if *elem == 0 {
                                            space = true;
                                        }
                                        if *elem == 2048 {
                                            win = true;
                                        }
                                    }
                                }
                                if space {
                                    generate_random(matrix);
                                    match update_file(1, matrix) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln!("Error updating file: {}", err);
                                            return;
                                        }
                                    }
                                } else if !check(matrix) {
                                    println!("\n    ----GAME OVER----");
                                    *matrix = [[0; 4]; 4];
                                    generate_random(matrix);
                                    generate_random(matrix);
                                    match update_file(1, matrix) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln!("Error updating file: {}", err);
                                            return;
                                        }
                                    }
                                    break;
                                }
                                if win {
                                    clear_screen();
                                    print_matrix(matrix);
                                    println!("\n     ----YOU WON! GOOD JOB!----");
                                    match update_file(0, matrix) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            eprintln!("Error updating file: {}", err);
                                            return;
                                        }
                                    }
                                    break;
                                }
                            }
                            KeyCode::Esc => {
                                // Handle Escape key press
                                clear_screen();
                                println!("Escape key pressed");
                                println!("\n     ---Exit game...");
                                break;
                            }
                            _ => (),
                        }
                    }
                }
            }
            Err(_) => {
                eprintln!("Error reading input");
                break;
            }
        }
    }
}

fn main() {
    let mut matrix: [[i32; 4]; 4] = [[0; 4]; 4];
    game_start(&mut matrix);
    print_instructions();
    game_logic(&mut matrix);
}
