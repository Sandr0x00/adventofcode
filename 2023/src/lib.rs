
use std::fs;

pub fn input(day: u8) -> String {
    let file = format!("src/day_{:0>2}/input.txt", day);

    if fs::metadata(&file).is_err() {
        println!("{file} not found.");
        return "".to_owned();
    }

    fs::read_to_string(file).unwrap()
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);

    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd_of_two_numbers(b, a % b)
}

pub fn print_solution<T: std::fmt::Display>(parts: &[T]) {
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