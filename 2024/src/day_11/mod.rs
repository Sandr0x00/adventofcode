use std::collections::HashMap;
use std::iter::successors;

fn len(n: u64) -> u32 {
    successors(Some(n), |&i| (i >= 10).then_some(i / 10)).count() as u32
}

pub fn solve(input: String) -> Vec<u64> {
    let mut stones: HashMap<u64, u64> = HashMap::new();

    for stone in input.split_ascii_whitespace() {
        *stones.entry(stone.parse().unwrap()).or_default() += 1;
    }

    let mut res_one = 0;
    let mut res_two = 0;

    for i in 0..75 {
        let mut new_stones: HashMap<u64, u64> = HashMap::new();
        let mut sum = 0;
        for (stone, amount) in &stones {
            if *stone == 0 {
                *new_stones.entry(1).or_default() += amount;
                sum += amount;
                continue;
            }

            let stone_len = len(*stone);
            if stone_len & 1 == 0 {
                // even
                let modulo = 10_u64.pow(stone_len / 2);
                *new_stones.entry(*stone / modulo).or_default() += amount;
                *new_stones.entry(*stone % modulo).or_default() += amount;
                sum += amount * 2;
            } else {
                // odd
                *new_stones.entry(*stone * 2024).or_default() += amount;
                sum += amount;
            }
        }
        stones = new_stones;
        if i == 24 {
            res_one = sum;
        }
        res_two = sum;
    }

    vec![res_one, res_two]
}

#[test]
fn test() {
    assert_eq!(solve("125 17".to_string())[0], 55312);

    assert_eq!(solve(aoc::input(11)), [183620, 220377651399268]);
}
