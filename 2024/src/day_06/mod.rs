use std::collections::HashSet;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum Direction {
    U,
    R,
    D,
    L,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Step {
    x: i32,
    y: i32,
    dir: Direction,
}

fn next(
    matrix: &[Vec<u8>],
    step: Step,
    bounds: &(i32, i32),
    distinct_fields: &mut HashSet<(i32, i32)>,
    loop_check_pos: (i32, i32),
    cache: &mut HashSet<Step>,
) -> u64 {
    // check if we already went into the possible right-turn
    if loop_check_pos != (-1, -1) {
        if cache.contains(&step) {
            // exit immediately when encountering a loop
            return 1;
        }
    } else {
        distinct_fields.insert((step.x, step.y));
    }
    cache.insert(step.clone());

    // next pos
    let next_pos = match step.dir {
        Direction::U => (step.x, step.y - 1),
        Direction::R => (step.x + 1, step.y),
        Direction::D => (step.x, step.y + 1),
        Direction::L => (step.x - 1, step.y),
    };

    // bounds check
    if next_pos.0 < 0 || next_pos.1 < 0 {
        return 0;
    }
    if next_pos.0 > bounds.0 || next_pos.1 > bounds.1 {
        return 0;
    }

    let step = if matrix[next_pos.1 as usize][next_pos.0 as usize] == b'#'
        || next_pos.0 == loop_check_pos.0 && next_pos.1 == loop_check_pos.1
    {
        // we ran into roadblock, turn right, but do not yet walk
        Step {
            x: step.x,
            y: step.y,
            dir: match step.dir {
                Direction::U => Direction::R,
                Direction::R => Direction::D,
                Direction::D => Direction::L,
                Direction::L => Direction::U,
            },
        }
    } else {
        // step in the previous direction
        Step {
            x: next_pos.0,
            y: next_pos.1,
            dir: step.dir,
        }
    };

    next(matrix, step, bounds, distinct_fields, loop_check_pos, cache)
}

fn find_start(matrix: &[Vec<u8>], bounds: &(i32, i32)) -> Step {
    for y in 0..bounds.1 + 1 {
        for x in 0..bounds.0 + 1 {
            if matrix[y as usize][x as usize] == b'^' {
                return Step {
                    x,
                    y,
                    dir: Direction::U,
                };
            }
        }
    }

    panic!("Start not found")
}

pub fn solve(input: String) -> Vec<u64> {
    let matrix = aoc::parse_matrix(input);
    let bounds: (i32, i32) = (matrix[0].len() as i32 - 1, matrix.len() as i32 - 1);

    let mut distinct_fields = HashSet::new();

    let start = find_start(&matrix, &bounds);
    let mut cache = HashSet::new();
    next(
        &matrix,
        start.clone(),
        &bounds,
        &mut distinct_fields,
        (-1, -1),
        &mut cache,
    );

    let cache_len = cache.len();
    // Brute forcing takes a few seconds in release mode, but works well enough
    let mut loop_cnt: u64 = 0;
    for (x, y) in distinct_fields.clone().iter() {
        if start.x == *x && start.y == *y {
            continue;
        }
        // no more dynamic resizing
        let mut c = HashSet::with_capacity(cache_len);
        loop_cnt += next(
            &matrix,
            start.clone(),
            &bounds,
            &mut distinct_fields,
            (*x, *y),
            &mut c,
        );
    }

    vec![distinct_fields.len() as u64, loop_cnt]
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
                .to_string()
        ),
        [41, 6]
    );

    // assert_eq!(solve(aoc::input(6)), [4602, 1703]);
}
