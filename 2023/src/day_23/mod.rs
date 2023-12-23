use std::collections::{HashSet, HashMap};
use std::collections::VecDeque;
use itertools::assert_equal;
use petgraph::Directed;
use petgraph::algo::tarjan_scc;
use petgraph::graphmap::DiGraphMap;
use petgraph_evcxr::draw_graph;
use pathfinding::prelude::*;

const DAY: u8 = 23;

const DEBUG: bool = false;

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
struct Pos {
    x: isize,
    y: isize,
    prev: (isize, isize)
}

#[allow(dead_code)]
pub fn solve() {
    let input = aoc::input(DAY);

    let mut matrix: Vec<Vec<u8>> = Vec::new();

    let mut graph = HashMap::<(isize, isize), Vec<(isize, isize)>>::new();

    let num = 1000;

    for line in input.lines() {
        matrix.push(line.as_bytes().to_vec());
    }
    let bounds: (isize, isize) = (matrix[0].len() as isize, matrix.len() as isize);

    let start = Pos{
        x: matrix[0].iter().position(|&p| p == b'.').unwrap() as isize,
        y: 0,
        prev: (-1,-1),
    };
    let target = Pos{
        x: matrix[bounds.1 as usize - 1].iter().position(|&p| p == b'.').unwrap() as isize,
        y: bounds.1 - 1,
        prev: (-1, -1)
    };

    let mut paths = topological_sort(&[start.clone()],
        |pos| {
            let mut v = Vec::new();
            for i in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let x = pos.x + i.0;
                let y = pos.y + i.1;
                let prev = (pos.x, pos.y);
                if x == -1 || x == bounds.0 || y == -1 || y == bounds.1 {
                    continue;
                }
                if pos.prev.0 == x && pos.prev.1 == y {
                    // enforce directedness
                    continue;
                }
                let m = matrix[y as usize][x as usize];
                let next = Pos{x, y, prev};
                if m == b'.' {
                    v.push(next.clone());
                }
                if pos.x < x && m == b'>' {
                    v.push(next.clone());
                }
                if pos.x > x && m == b'<' {
                    v.push(next.clone());
                }
                if pos.y > y && m == b'^' {
                    v.push(next.clone());
                }
                if pos.y < y && m == b'v' {
                    v.push(next.clone());
                }
            }
            v//.into_iter().map(|p| (p,1))
        },
        // |&pos| {
        //     // ((target.0.abs_diff(pos.0) + target.1.abs_diff(pos.1)) / 3).wrapping_add(usize::MAX)
        //     ((target.0.abs_diff(pos.0) + target.1.abs_diff(pos.1)) as isize * 3)
        // },
        // |&pos| {
        //     pos.0 == target.0 && pos.1 == target.1
        // }
    );

    assert_eq!(paths.iter().len(), 1);
    for ps in paths {
        let mut dist = HashMap::with_capacity(ps.len());
        for p in &ps {
            if p.prev.0 == -1 {
                continue;
            }
            graph.entry((p.x, p.y)).or_default();
            graph.entry(p.prev).and_modify(|pos| pos.push((p.x, p.y))).or_default();
            // println!("{:?}", p);
            dist.entry((p.x, p.y)).or_insert(1);
        }
        println!("grpah done");
        for p in ps {
            // println!("{:?}",p);
            for (x,y) in &graph[&(p.x, p.y)] {
                let new = dist[&(p.x, p.y)] + 1;
                if dist[&(*x, *y)] < new {
                    dist.entry((*x,*y)).and_modify(|v| *v = new);
                }
            }
        //     graph.entry((p.x, p.y)).and_modify(|pos| pos.push((p.x, p.y))).or_insert(Vec::new());
        //     println!("{:?}", p);
        }

        println!("{:?}", dist[&(target.x, target.y)]);
    }

    // println!("{:?}", paths.unwrap());
//
    // while let Some(n) = paths.next() {
    //     println!("{:?}", n);
    // }

    aoc::print_solution(DAY, &[0]);
}
