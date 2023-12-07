
use std::fs;

pub fn input(day: u8) -> String {
    let file = format!("input_{:0>2}.txt", day);

    fs::read_to_string(file).unwrap()
}