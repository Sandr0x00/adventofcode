use std::collections::VecDeque;

const DAY: u8 = 9;

pub fn solve() {
    let input = aoc::input(DAY);

    let mut sum_one = 0;
    let mut sum_two = 0;
    for line in input.lines() {
        let mut sequence: Vec<_> = line.split_ascii_whitespace().map(|d| d.parse::<isize>().unwrap()).collect();

        let mut list_end = VecDeque::<isize>::new();
        let mut list_start = VecDeque::<isize>::new();
        let mut modifier = -1;
        while !sequence.iter().all(|x| *x == 0) {
            modifier *= -1;
            list_end.push_front(modifier * sequence[sequence.len() - 1]);
            list_start.push_front(sequence[0]);
            let mut new_sequence = Vec::new();
            for i in 0..sequence.len() - 1 {
                new_sequence.push(sequence[i] - sequence[i+1]);
            }
            sequence = new_sequence;
        }
        sum_one += list_end.iter().fold(0, |acc, num| acc + num);
        sum_two += list_start.iter().fold(0, |acc, num| acc + num);
    }

    aoc::print_solution(DAY, &[sum_one, sum_two]);
}