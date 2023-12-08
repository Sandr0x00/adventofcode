use aoc;
use std::collections::HashSet;

fn get_matching_value(part1: &str, part2: &str, part3: &str) -> u32 {
    let mut possibles = HashSet::new();
    for p in part1.chars() {
        if part2.contains(p) {
            possibles.insert(p);
        }
    }
    if possibles.len() == 1 {
        for p in possibles.clone() {
            if p.is_lowercase() {
                return p as u32 - 96;
            }
            if p.is_uppercase() {
                return p as u32 - 38;
            }
        }
    }
    for p in possibles {
        if part3.contains(p) {
            if p.is_lowercase() {
                return p as u32 - 96;
            }
            if p.is_uppercase() {
                return p as u32 - 38;
            }
        }
    }
    unreachable!();
}

pub fn solve() {
    let input = aoc::input(3);

    let mut score_one = 0;
    let mut score_two = 0;
    let mut parts_two1: &str = "";
    let mut parts_two2: &str = "";
    let mut parts_two3: &str = "";
    for (i, line) in input.lines().enumerate() {
        let parts = line.split_at(line.len() / 2);

        let idx = (i + 1) % 3;
        if idx == 1 {
            parts_two1 = line;
        } else if idx == 2 {
            parts_two2 = line;
        } else if idx == 0 {
            parts_two3 = line;
            score_two += get_matching_value(parts_two1, parts_two2, parts_two3);
            parts_two1 = "";
            parts_two2 = "";
            parts_two3 = "";
        }
        score_one += get_matching_value(parts.0, parts.1, "");
    }

    aoc::print_solution(3, &[score_one, score_two]);
}