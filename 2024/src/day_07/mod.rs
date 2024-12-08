fn rek(target: u64, status: u64, remaining_items: Vec<u64>) -> bool {
    if status > target {
        // early exit condition
        return false;
    }
    if remaining_items.is_empty() {
        return target == status;
    }
    rek(
        target,
        status + remaining_items[0],
        remaining_items[1..].to_vec(),
    ) || rek(
        target,
        status * remaining_items[0],
        remaining_items[1..].to_vec(),
    )
}

fn rek2(target: u64, status: u64, remaining_items: Vec<u64>) -> bool {
    if status > target {
        // early exit condition
        return false;
    }
    if remaining_items.is_empty() {
        return target == status;
    }
    rek2(
        target,
        status + remaining_items[0],
        remaining_items[1..].to_vec(),
    ) || rek2(
        target,
        status * remaining_items[0],
        remaining_items[1..].to_vec(),
    ) || rek2(
        target,
        format!("{}{}", status, remaining_items[0]).parse().unwrap(),
        remaining_items[1..].to_vec(),
    )
}

pub fn solve(input: String) -> Vec<u64> {
    let lines: Vec<_> = input.lines().collect();

    let mut part_one = 0;
    let mut part_two = 0;
    for line in lines.iter() {
        let (target, eq) = line.split_once(": ").unwrap();
        let target: u64 = target.parse().unwrap();
        let items: Vec<u64> = eq.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();

        if rek(target, items[0], items[1..].to_vec()) {
            part_one += target;
        }
        if rek2(target, items[0], items[1..].to_vec()) {
            part_two += target;
        }
    }

    vec![part_one, part_two]
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
                .to_string()
        ),
        [3749, 11387]
    );

    assert_eq!(solve(aoc::input(7)), [2299996598890, 362646859298554]);
}
