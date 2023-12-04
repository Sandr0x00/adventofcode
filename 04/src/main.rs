use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();

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
    println!("Part One {}", sum);
    println!("Part Two {}", winnings.iter().sum::<u32>());
}
