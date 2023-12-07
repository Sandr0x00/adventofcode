use y2022;

pub fn solve() {
    let input = y2022::input(2);

    let mut score_one: isize = 0;
    let mut score_two: isize = 0;
    for line in input.lines() {
        let row: Vec<_> = line.split_ascii_whitespace().collect();
        let (a, b) = ((row[0].as_bytes()[0] - 64) as isize, (row[1].as_bytes()[0] - 87) as isize);

        // Part One
        // add own number
        score_one += b;

        if a == b {
            // draw
            score_one += 3;
        } else {
            if b - a == 1 {
                // scissors > paper | paper > rock
                score_one += 6
            }
            if b - a == -2 {
                // rock > scissors
                score_one += 6
            }
        }

        // Part Two
        score_two += match b {
            1 => match a {
                1 => 3,
                2 => 1,
                3 => 2,
                _ => unreachable!(),
            },
            2 => 3 + a,
            3 => 6 + match a {
                1 => 2,
                2 => 3,
                3 => 1,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    println!("Part One {}", score_one);
    println!("Part Two {}", score_two);
}
