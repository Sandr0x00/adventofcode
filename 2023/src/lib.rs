
use std::fs;

pub fn input(day: u8) -> String {
    let file = format!("src/day_{:0>2}/input.txt", day);

    fs::read_to_string(file).unwrap()
}

pub fn print_solution<T: std::fmt::Display>(day: u8, parts: &[T]) {
    println!();
    println!("Day {day}");
    for (i, part) in parts.iter().enumerate() {
        match i {
            0 => println!("Part One {part}"),
            1 => println!("Part Two {part}"),
            _ => unreachable!(),
        }
    }
}