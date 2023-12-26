use std::collections::HashSet;
use std::collections::VecDeque;
use petgraph::Directed;
use petgraph::graphmap::GraphMap;
use petgraph_evcxr::draw_graph;
use pathfinding::prelude::astar;

const DEBUG: bool = false;

fn try_step(pos: (usize, usize), bounds: (usize, usize)) {
    let mut steps = Vec::new();
    if pos.0 > 0 {
        steps.push((pos.0 - 1, pos.1))
    }
}

fn step(matrix: &[Vec<u8>], positions: HashSet<(usize, usize)>, bounds: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut new_positions = HashSet::new();
    for p in positions.iter() {
        for i in [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)] {
            let dir = (p.0.wrapping_add(i.0), p.1.wrapping_add(i.1));
            if dir.0 == usize::MAX || dir.0 == bounds.0
                || dir.1 == usize::MAX || dir.1 == bounds.1 {
                continue;
            }
            if matrix[dir.1][dir.0] == b'.' {
                new_positions.insert(dir);
            }
        }
    }
    new_positions
}

fn step2(matrix: &[Vec<u8>], positions: HashSet<(usize, usize)>, bounds: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut new_positions = HashSet::new();
    for p in positions.iter() {
        for i in [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)] {
            let dir = (p.0.wrapping_add(i.0), p.1.wrapping_add(i.1));
            // if dir.0 == usize::MAX {
            //     dir.0 = bounds.0 - 1;
            // } else if dir.0 == bounds.0 {
            //     dir.0 = 0;
            // }
            // if dir.1 == usize::MAX {
            //     dir.1 = bounds.1 - 1;
            // } else if dir.1 == bounds.1 {
            //     dir.1 = 0
            // }
            if matrix[dir.1 % bounds.1][dir.0 % bounds.0] == b'.' {
                new_positions.insert(dir);
            }
        }
    }
    new_positions
}

pub fn solve(input: String) {
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    let mut positions = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        let mut inner = Vec::new();
        for (x, ch) in line.as_bytes().iter().enumerate() {
            if *ch == b'S' {
                inner.push(b'.');
                positions.insert((x, y));
            } else {
                inner.push(*ch);
            }
        }
        matrix.push(inner);
    }
    let bounds: (usize, usize) = (matrix[0].len(), matrix.len());


    // for _ in 0..64 {
    //     positions = step(&matrix, positions, bounds);
    // }
    // println!("{:?}", positions.len());
    for _ in 0..10 {
        positions = step2(&matrix, positions, bounds);
    }
    println!("{:?}", positions.len());


    aoc::print_solution(&[0]);
}
