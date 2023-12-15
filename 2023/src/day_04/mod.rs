
const DAY: u8 = 4;

#[allow(dead_code)]
pub fn solve() {
    let contents = aoc::input(DAY);

    let mut sum: u32 = 0;
    // every card has 1 original
    let mut winnings = vec![1; contents.lines().count()];

    for (i, line) in contents.lines().enumerate() {
        let w_n: Vec<&str> = line[10..].split('|').collect();
        let winners: Vec<i32> = w_n[0].split(' ').filter_map(|w| w.parse::<i32>().ok()).collect();
        let numbers: Vec<i32> = w_n[1].split(' ').filter_map(|w| w.parse::<i32>().ok()).collect();

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

    aoc::print_solution(DAY, &[sum, winnings.iter().sum::<u32>()])
}
