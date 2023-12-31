
pub fn solve(input: String) {
    let mut sum: u32 = 0;
    // every card has 1 original
    let mut winnings = vec![1; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let (w, n) = line[10..].split_once('|').unwrap();
        let winners: Vec<i32> = w.split(' ').filter_map(|w| w.parse().ok()).collect();
        let numbers: Vec<i32> = n.split(' ').filter_map(|w| w.parse().ok()).collect();

        let mut idx: isize = -1;
        for winner in winners {
            if numbers.contains(&winner) {
                idx += 1;
                // add more future copies according to how many copies we have right now
                winnings[i + 1 + (idx as usize)] += winnings[i]
            }
        }
        // points get doubled for each match
        if idx >= 0 {
            sum += u32::pow(2, idx as u32);
        }
    }

    aoc::print_solution(&[
        sum,
        winnings.iter().sum::<u32>()
    ])
}
