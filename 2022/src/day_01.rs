use y2022;

pub fn solve() {
    let input = y2022::input(1);

    let mut elves = vec![0];
    let mut elf = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves.push(0);
            elf += 1;
            continue
        }
        elves[elf] += line.parse::<usize>().unwrap();
    }

    elves.sort();
    elves.reverse();
    println!("Part One {}", elves[0]);
    println!("Part Two {}", elves[..3].iter().sum::<usize>());
}
