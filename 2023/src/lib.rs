
use std::fs;

pub fn input(day: u8) -> String {
    let file = format!("src/day_{:0>2}/input.txt", day);

    if fs::metadata(&file).is_err() {
        println!("{file} not found.");
        return "".to_owned();
    }

    fs::read_to_string(file).unwrap()
}

pub fn print_solution<T: std::fmt::Display>(day: u8, parts: &[T]) {
    println!("\nDay {day}");
    print!("Part One ");
    if parts.is_empty() {
        println!("not yet solved");
    } else {
        println!("{}", parts[0]);
    }
    print!("Part Two ");
    if parts.len() > 1 {
        println!("{}", parts[1]);
    } else {
        println!("not yet solved");
    }
}

pub fn todo(day: u8) {
    println!("\nDay {day}\nNot yet solved.");
}

pub fn print_matrix(matrix: &[Vec<u8>]) {
    println!();
    for row in matrix {
        for c in row {
            print!("{}", *c as char);
        }
        println!()
    }
}