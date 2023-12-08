use aoc;

pub fn solve() {
    let input = aoc::input(1);

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

    aoc::print_solution(1, &[elves[0], elves[..3].iter().sum::<usize>()])
}
