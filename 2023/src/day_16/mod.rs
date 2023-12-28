use std::collections::HashSet;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum Direction {
    U,
    R,
    D,
    L,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Beam {
    x: usize,
    y: usize,
    dir: Direction,
}

fn step(matrix: &[Vec<u8>], cache: &mut HashSet<Beam>, beam: Beam, bounds: (usize, usize), solution: &mut HashSet<(usize, usize)>) {
    match matrix[beam.y][beam.x] {
        b'.' => rek(matrix, cache, beam, bounds, solution),
        b'|' => match beam.dir {
            Direction::D | Direction::U => rek(matrix, cache, beam, bounds, solution),
            Direction::L | Direction::R => {
                rek(matrix, cache, Beam{dir: Direction::U, ..beam}, bounds, solution);
                rek(matrix, cache, Beam{dir: Direction::D, ..beam}, bounds, solution);
            }
        },
        b'-' => match beam.dir {
            Direction::L | Direction::R => rek(matrix, cache, beam, bounds, solution),
            Direction::D | Direction::U => {
                rek(matrix, cache, Beam{dir: Direction::L, ..beam}, bounds, solution);
                rek(matrix, cache, Beam{dir: Direction::R, ..beam}, bounds, solution);
            }
        },
        b'\\' => rek(matrix, cache, Beam{dir: match beam.dir {
            Direction::L => Direction::U,
            Direction::R => Direction::D,
            Direction::D => Direction::R,
            Direction::U => Direction::L,
        }, ..beam}, bounds, solution),
        b'/' => rek(matrix, cache, Beam{dir: match beam.dir {
            Direction::L => Direction::D,
            Direction::R => Direction::U,
            Direction::D => Direction::L,
            Direction::U => Direction::R,
        }, ..beam}, bounds, solution),
        _ => unreachable!(),
    }
}

fn rek(matrix: &[Vec<u8>], cache: &mut HashSet<Beam>, beam: Beam, bounds: (usize, usize), solution: &mut HashSet<(usize, usize)>) {
    if cache.contains(&beam) {
        return;
    }
    cache.insert(beam.clone());
    solution.insert((beam.x, beam.y));

    let next_beam = match beam.dir {
        Direction::U => match beam.y {
            0 => None,
            _ => Some(Beam{y: beam.y - 1, ..beam}),
        },
        Direction::R => match beam.x {
            b if b == bounds.0 => None,
            _ => Some(Beam{x: beam.x + 1, ..beam}),
        },
        Direction::D => match beam.y {
            b if b == bounds.1 => None,
            _ => Some(Beam{y: beam.y + 1, ..beam}),
        },
        Direction::L => match beam.x {
            0 => None,
            _ => Some(Beam{x: beam.x - 1, ..beam}),
        },
    };

    if let Some(b) = next_beam {
        step(matrix, cache, b, bounds, solution);
    }
}

pub fn solve(input: String) {
    let matrix = aoc::parse_matrix(input);
    let bounds = (matrix[0].len() - 1, matrix.len() - 1);

    let mut solution_two = 0;

    let mut cache = HashSet::new();
    let mut solution = HashSet::new();
    let beam = Beam{x: 0, y: 0, dir: Direction::R};
    step(&matrix, &mut cache, beam, bounds, &mut solution);
    let solution_one = solution.len();

    for y in 0..bounds.1 + 1 {
        for b in [
            Beam{x: 0,        y, dir: Direction::R},
            Beam{x: bounds.0, y, dir: Direction::L},
        ] {
            cache = HashSet::new();
            solution = HashSet::new();
            step(&matrix, &mut cache, b, bounds, &mut solution);
            if solution.len() > solution_two {
                solution_two = solution.len()
            }
        }
    }

    for x in 1..bounds.0 + 1 {
        for b in [
            Beam{x, y: 0,        dir: Direction::D},
            Beam{x, y: bounds.1, dir: Direction::U},
        ] {
            cache = HashSet::new();
            solution = HashSet::new();
            step(&matrix, &mut cache, b, bounds, &mut solution);
            if solution.len() > solution_two {
                solution_two = solution.len()
            }
        }
    }

    aoc::print_solution(&[solution_one, solution_two]);
}
