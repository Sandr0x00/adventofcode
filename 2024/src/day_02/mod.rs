fn is_safe(line: &Vec<i32>) -> bool {
    let inc = line[0] < line[line.len() - 1];
    let mut last = line[0];

    for num in line[1..].iter() {
        if (inc && *num < last)
            || (!inc && *num > last)
            || ((last - *num).abs() > 3)
            || ((last - *num).abs() < 1)
        {
            return false;
        }
        last = *num;
    }
    true
}

pub fn solve(input: String) -> Vec<u64> {
    let lines: Vec<_> = input.lines().collect();

    let mut safe_cnt_one = 0;
    let mut safe_cnt_two = 0;
    for line in lines.iter() {
        let line: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let safe_one = is_safe(&line);
        let mut safe_two = safe_one;
        if !safe_two {
            for i in 0..line.len() {
                let filtered_line: Vec<_> = [&line[..i], &line[i + 1..]].concat();
                if is_safe(&filtered_line) {
                    safe_two = true;
                    break;
                }
            }
        }

        safe_cnt_one += safe_one as u64;
        safe_cnt_two += safe_two as u64;
    }

    vec![safe_cnt_one, safe_cnt_two]
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
                .to_string()
        ),
        vec![2, 4]
    );
}
