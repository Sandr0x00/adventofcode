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
    let data_one = &input
        .lines()
        .map(|l| {
            l[10..]
                .split_ascii_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    assert_eq!(data_one[0].len(), data_one[1].len());
    for i in 0..data_one[0].len() {
        res_one *= ways_to_win(data_one[0][i], data_one[1][i]);
    }

    let data_two = input
        .lines()
        .map(|l| {
            l[10..]
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    println!("Part One {}", res_one);
    println!("Part Two {}", ways_to_win(data_two[0], data_two[1]));
}
