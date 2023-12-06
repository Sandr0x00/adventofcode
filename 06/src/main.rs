use std::fs;

fn ways_to_win(time: usize, record: usize) -> usize {
    let mut min = 0;
    let mut max = 0;
    for i in 0..time {
        let distance = i * (time - i);
        if distance > record {
            min = i;
            break;
        }
    }
    for i in (0..time).rev() {
        let distance = i * (time - i);
        if distance > record {
            max = i;
            break;
        }
    }

    max - min + 1
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut res_one = 1;
    if let [times_one, records_one] = &input.lines().map(|l| l[10..].split(' ').filter_map(|w| w.parse::<usize>().ok()).collect::<Vec<_>>()).collect::<Vec<_>>()[0..2] {
        assert!(times_one.len() == records_one.len());
        for i in 0..times_one.len() {
            res_one *= ways_to_win(times_one[i], records_one[i]);
        }
    }
    println!("Part One {}", res_one);

    if let [time_two, record_two] = input.lines().map(|l| l[10..].chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<usize>().unwrap()).collect::<Vec<_>>()[0..2] {
        println!("Part Two {}", ways_to_win(time_two, record_two));
    }
}
