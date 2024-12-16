use regex::Regex;

const COST_A: u64 = 3;
const COST_B: u64 = 1;

fn linalg(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> u64 {
    let det = ax * by - ay * bx;
    let det_a = px * by - py * bx;
    let det_b = py * ax - px * ay;

    if det_a % det == 0 && det_b % det == 0 {
        let a = det_a / det;
        let b = det_b / det;
        return a as u64 * COST_A + b as u64 * COST_B;
    }

    0
}

pub fn solve(input: String) -> Vec<u64> {
    let machines: Vec<&str> = input.split("\n\n").collect();

    let mut token_one = 0;
    let mut token_two = 0;
    for machine in machines {
        let re = Regex::new(
            r"Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)\nButton B: X\+(?<bx>\d+), Y\+(?<by>\d+)\nPrize: X=(?<px>\d+), Y=(?<py>\d+)",
        )
        .unwrap();

        let caps = re.captures(machine).unwrap();
        let ax: i64 = caps["ax"].parse().unwrap();
        let ay: i64 = caps["ay"].parse().unwrap();
        let bx: i64 = caps["bx"].parse().unwrap();
        let by: i64 = caps["by"].parse().unwrap();
        let px: i64 = caps["px"].parse().unwrap();
        let py: i64 = caps["py"].parse().unwrap();

        token_one += linalg(ax, ay, bx, by, px, py);
        token_two += linalg(ax, ay, bx, by, px + 10000000000000, py + 10000000000000);
    }

    vec![token_one, token_two]
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
                .to_string()
        )[0],
        480
    );

    assert_eq!(solve(aoc::input(13)), [32041, 95843948914827]);
}
