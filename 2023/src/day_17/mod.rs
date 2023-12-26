use pathfinding::prelude::astar;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    N, // none
    U,
    R,
    D,
    L,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Step {
    x: isize,
    y: isize,
    dir: Direction,
    amount: usize,
}

fn add(v: &mut Vec<(Step, usize)>, matrix: &[Vec<u8>], bounds: (isize, isize), p: Step, amount_max: usize) {
    if p.amount == amount_max {
        return
    }
    if p.x < 0 || p.y < 0 || p.x > bounds.0 || p.y > bounds.1 {
        return
    }

    v.push((p, (matrix[p.y as usize][p.x as usize]) as usize));
}

fn options(v: &mut Vec<(Step, usize)>, matrix: &[Vec<u8>], bounds: (isize, isize), p: Step, amount_min: usize, amount_max: usize) {
    let amount = p.amount + 1;

    match p.dir {
        Direction::L | Direction::R => {
            add(v, matrix, bounds, match p.dir {
                Direction::L => Step{x: p.x - 1, amount, ..p},
                Direction::R => Step{x: p.x + 1, amount, ..p},
                _ => unreachable!(),
            }, amount_max);
            if amount_min <= amount {
                add(v, matrix, bounds, Step{y: p.y - 1, dir: Direction::U, amount: 0, ..p}, amount_max);
                add(v, matrix, bounds, Step{y: p.y + 1, dir: Direction::D, amount: 0, ..p}, amount_max);
            }
        },
        Direction::U | Direction::D => {
            add(v, matrix, bounds, match p.dir {
                Direction::U => Step{y: p.y - 1, amount, ..p},
                Direction::D => Step{y: p.y + 1, amount, ..p},
                _ => unreachable!(),
            }, amount_max);
            if amount_min <= amount {
                add(v, matrix, bounds, Step{x: p.x - 1, dir: Direction::L, amount: 0, ..p}, amount_max);
                add(v, matrix, bounds, Step{x: p.x + 1, dir: Direction::R, amount: 0, ..p}, amount_max);
            }
        },
        Direction::N => {
            add(v, matrix, bounds, Step{x: 0, y: 1, dir: Direction::D, amount: 1}, amount_max);
            add(v, matrix, bounds, Step{x: 1, y: 0, dir: Direction::R, amount: 1}, amount_max);
        }
    }
}

const DEBUG: bool = false;

pub fn solve(input: String) {
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        matrix.push(line.as_bytes().iter().map(|c| c - b'0').collect());
    }
    let bounds: (isize, isize) = ((matrix[0].len() - 1) as isize, (matrix.len() - 1) as isize);

    if DEBUG {
        aoc::print_matrix(&matrix);
    }

    let result_one = astar(&Step{x: 0, y: 0, dir: Direction::N, amount: 0},
        |&p| {
            let mut v: Vec<(Step, usize)> = Vec::new();
            options(&mut v, &matrix, bounds, p, 0, 3);
            v
        },
        |&p| {
            (bounds.0.abs_diff(p.x) + bounds.1.abs_diff(p.y)) / 4
        },
        |&p| {
            p.x == bounds.0 && p.y == bounds.0
        }
    ).unwrap();

    let result_two = astar(&Step{x: 0, y: 0, dir: Direction::N, amount: 0},
        |&p| {
            let mut v: Vec<(Step, usize)> = Vec::new();
            options(&mut v, &matrix, bounds, p, 4, 10);
            v
        },
        |&p| {
            (bounds.0.abs_diff(p.x) + bounds.1.abs_diff(p.y)) / 4
        },
        |&p| {
            p.x == bounds.0 && p.y == bounds.0 && p.amount >= 4
        }
    ).unwrap();

    if DEBUG {
        for y in 0..bounds.1 {
            for x in 0..bounds.0 {
                matrix[y as usize][x as usize] += b'0';
            }
        }
        for r in result_two.0 {
            matrix[r.y as usize][r.x as usize] = match r.dir {
                Direction::D => b'v',
                Direction::R => b'>',
                Direction::U => b'^',
                Direction::L => b'<',
                Direction::N => b'O',
            };
        }
        aoc::print_matrix(&matrix);
    }

    aoc::print_solution(&[result_one.1, result_two.1]);
}
