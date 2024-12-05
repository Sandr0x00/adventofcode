use regex::Regex;

pub fn solve(input: String) -> Vec<u64> {
    let re = Regex::new(
        r"(?<mul>mul)\((?<first>\d{1,3}),(?<second>\d{1,3})\)|(?<do>do)\(\)|(?<dont>don't)\(\)",
    )
    .unwrap();

    let mut res_one = 0;
    let mut res_two = 0;
    let mut enabled = true;
    for cap in re.captures_iter(&input) {
        if cap.name("mul").is_some() {
            let first = cap["first"].parse::<i32>().unwrap();
            let second = cap["second"].parse::<i32>().unwrap();
            let res = first * second;
            res_one += res;
            if enabled {
                res_two += res;
            }
        } else if cap.name("do").is_some() {
            enabled = true;
        } else if cap.name("dont").is_some() {
            enabled = false;
        }
    }

    vec![res_one as u64, res_two as u64]
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()
        )[0],
        161
    );

    assert_eq!(
        solve(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()
        )[1],
        48
    );

    assert_eq!(solve(aoc::input(3)), [174103751, 100411201]);
}
