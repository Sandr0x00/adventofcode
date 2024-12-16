use pathfinding::prelude::yen;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    U,
    R,
    D,
    L,
}

#[derive(Debug, Clone, Copy)]
struct Step {
    x: i32,
    y: i32,
    dir: Direction,
}

// we only want x and y to be considered in eq / hash
impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Step {}

impl Hash for Step {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Step {
    fn step(&self, dir: Direction) -> Step {
        match dir {
            Direction::L => Step {
                x: self.x - 1,
                y: self.y,
                dir: Direction::L,
            },
            Direction::R => Step {
                x: self.x + 1,
                y: self.y,
                dir: Direction::R,
            },
            Direction::U => Step {
                x: self.x,
                y: self.y - 1,
                dir: Direction::U,
            },
            Direction::D => Step {
                x: self.x,
                y: self.y + 1,
                dir: Direction::D,
            },
        }
    }
}

fn add(v: &mut Vec<(Step, u32)>, m: &[Vec<u8>], b: (i32, i32), p: Step, val: u32) {
    if p.x < 0 || p.y < 0 || p.x > b.0 || p.y > b.1 {
        return;
    }
    if m[p.y as usize][p.x as usize] == b'#' {
        // cannot walk into wall
        return;
    }

    v.push((p, val));
}

fn options(v: &mut Vec<(Step, u32)>, m: &[Vec<u8>], b: (i32, i32), p: Step) {
    add(v, m, b, p.step(p.dir), 1);
    match p.dir {
        Direction::L | Direction::R => {
            add(v, m, b, p.step(Direction::U), 1001);
            add(v, m, b, p.step(Direction::D), 1001);
        }
        Direction::U | Direction::D => {
            add(v, m, b, p.step(Direction::L), 1001);
            add(v, m, b, p.step(Direction::R), 1001);
        }
    }
}

pub fn solve(input: String) -> Vec<u64> {
    let (matrix, bounds) = aoc::parse_matrix(input);

    let mut start = Step {
        x: 0,
        y: 0,
        dir: Direction::D,
    };
    let mut end = Step {
        x: 0,
        y: 0,
        dir: Direction::D,
    };
    for y in 1..bounds.1 {
        for x in 1..bounds.0 {
            if matrix[y as usize][x as usize] == b'S' {
                start = Step {
                    x,
                    y,
                    dir: Direction::R,
                };
            } else if matrix[y as usize][x as usize] == b'E' {
                end = Step {
                    x,
                    y,
                    dir: Direction::D,
                };
            }
        }
    }

    let shortest_paths = yen(
        &start,
        |&p| {
            let mut v: Vec<(Step, u32)> = Vec::new();
            options(&mut v, &matrix, bounds, p);
            v
        },
        |&p| p == end,
        // guess 10 paths, seems okay :shrug:
        10,
    );

    let mut score = 0;
    let mut tiles = HashSet::new();
    for (path, path_score) in shortest_paths {
        if score == 0 {
            // first
            score = path_score;
        } else if score < path_score {
            // no longer shortest
            break;
        }

        for s in path {
            tiles.insert(s);
        }
    }

    vec![score as u64, tiles.len() as u64]
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
                .to_string()
        ),
        vec![7036, 45]
    );

    assert_eq!(
        solve(
            "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
                .to_string()
        ),
        vec![11048, 64]
    );

    assert_eq!(solve(aoc::input(16)), [108504, 538]);
}
