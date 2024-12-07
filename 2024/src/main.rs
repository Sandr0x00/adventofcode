use std::env;
use std::io::{self, Write};
use std::process::Command;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
// mod day_08;
// mod day_09;
// mod day_10;
// mod day_11;
// // mod day_12;
// mod day_13;
// mod day_14;
// mod day_15;
// mod day_16;
// mod day_17;
// mod day_18;
// // mod day_19; JavaScript
// mod day_20;
// // mod day_21;
// // mod day_22;
// mod day_23;
// mod day_24;
// mod day_25;
use std::time::Instant;

#[allow(dead_code)]
fn run_non_rust(day: u8, language: &str) {
    let file = format!(
        "src/day_{day:0>2}/solve.{}",
        match language {
            "python" => "py",
            "node" => "js",
            _ => todo!(),
        }
    );
    let output = Command::new(language).arg(file).output().unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
}

fn run(day: u8) {
    println!("\nDay {day}");

    let input = aoc::input(day);

    let start_time = Instant::now();
    let res = match day {
        1 => day_01::solve(input),
        2 => day_02::solve(input),
        3 => day_03::solve(input),
        4 => day_04::solve(input),
        5 => day_05::solve(input),
        6 => day_06::solve(input),
        7 => day_07::solve(input),
        //  8 => day_08::solve(input),
        //  9 => day_09::solve(input),
        // 10 => day_10::solve(input),
        // 11 => day_11::solve(input),
        // // 12 => day_12::solve(input),
        // 13 => day_13::solve(input),
        // 14 => day_14::solve(input),
        // 15 => day_15::solve(input),
        // 16 => day_16::solve(input),
        // 17 => day_17::solve(input),
        // 18 => day_18::solve(input),
        // 19 => run_non_rust(day, "node"),
        // 20 => day_20::solve(input),
        // // 21 => day_21::solve(input),
        // // 22 => day_22::solve(input: String),
        // 23 => day_23::solve(input),
        // 24 => day_24::solve(input),
        // 25 => day_25::solve(input),
        _ => unreachable!(),
    };
    let took = start_time.elapsed();

    aoc::print_solution(&res);
    println!("Took {took:?}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for a in args[1..].iter() {
        run(a.parse::<u8>().unwrap());
    }
}
