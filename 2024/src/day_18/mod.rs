use pathfinding::prelude::astar;
use std::collections::HashSet;

fn add(
    v: &mut Vec<((isize, isize), usize)>,
    blocked: &HashSet<(isize, isize)>,
    bounds: (isize, isize),
    p: (isize, isize),
) {
    if p.0 < 0 || p.1 < 0 || p.0 > bounds.0 || p.1 > bounds.1 {
        return;
    }

    if blocked.contains(&(p.0, p.1)) {
        return;
    }

    v.push((p, 1));
}

fn solve_special(input: &str, bytes: usize, bounds: (isize, isize)) -> (usize, String) {
    let lines: Vec<_> = input.lines().collect();

    let mut blocked = HashSet::new();
    let mut res_one = 0;

    for (i, line) in lines.iter().enumerate() {
        let (x, y) = line.split_once(",").unwrap();
        blocked.insert((x.parse().unwrap(), y.parse().unwrap()));
        if i + 1 < bytes {
            continue;
        }

        let result = astar(
            &(0, 0),
            |&p| {
                let mut v: Vec<((isize, isize), usize)> = Vec::new();
                add(&mut v, &blocked, bounds, (p.0 - 1, p.1));
                add(&mut v, &blocked, bounds, (p.0 + 1, p.1));
                add(&mut v, &blocked, bounds, (p.0, p.1 - 1));
                add(&mut v, &blocked, bounds, (p.0, p.1 + 1));
                v
            },
            |&p| (bounds.0.abs_diff(p.0) + bounds.1.abs_diff(p.1)) / 4,
            |&p| p.0 == bounds.0 && p.1 == bounds.1,
        );
        if result.is_none() {
            return (res_one, line.to_string());
        }

        if i + 1 == bytes {
            res_one = result.unwrap().1;
        }
    }

    (res_one, "".to_string())
}

pub fn solve(input: String) -> Vec<u64> {
    let (res_one, res_two) = solve_special(&input, 1024, (70, 70));

    aoc::print_solution(&[format!("{}", res_one), res_two]);
    vec![0, 0]
}

#[test]
fn test() {
    assert_eq!(
        solve_special(
            "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
            12,
            (6, 6)
        ),
        (22, "6,1".to_string())
    );

    assert_eq!(
        solve_special(&aoc::input(18), 1024, (70, 70)),
        (354, "36,17".to_string())
    );
}
