use petgraph::graphmap::GraphMap;
use petgraph::prelude::Directed;
use petgraph::visit::Bfs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

#[allow(unused_imports)]
use petgraph::dot::{Dot, Config};

fn add(matrix: &[Vec<u8>], g: &mut GraphMap<(i32, i32), usize, Directed>, bounds: (i32, i32), p1: (i32, i32), p2: (i32, i32)) {
    if p1.0 < 0 || p1.1 < 0 || p1.0 > bounds.0 || p1.1 > bounds.1 {
        return;
    }
    if p2.0 < 0 || p2.1 < 0 || p2.0 > bounds.0 || p2.1 > bounds.1 {
        return;
    }

    // for debug fields
    if matrix[p1.1 as usize][p1.0 as usize] == b'.' {
        return
    }
    if matrix[p2.1 as usize][p2.0 as usize] == b'.' {
        return
    }

    // trail has to increase by exactly one
    if matrix[p1.1 as usize][p1.0 as usize] + 1 == matrix[p2.1 as usize][p2.0 as usize] {
        g.add_edge((p1.0, p1.1), (p2.0, p2.1), 1);
    }
}

fn find_all_paths(
    graph: &GraphMap<(i32, i32), usize, Directed>,
    start: (i32, i32),
    end: &(i32, i32),
) -> u64 {
    let mut path_count = 0;
    #[allow(clippy::type_complexity)]
    let mut queue: Vec<((i32, i32), Vec<(i32, i32)>)> = vec![(start, vec![start])];

    while let Some((current_node, path)) = queue.pop() {
        if current_node == *end {
            path_count += 1;
        } else {
            for neighbor in graph.neighbors(current_node) {
                if !path.contains(&neighbor) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    queue.push((neighbor, new_path));
                }
            }
        }
    }

    path_count
}

pub fn solve(input: String) -> Vec<u64> {
    let (matrix, bounds) = aoc::parse_matrix(input);

    let mut starts = Vec::new();
    let mut ends = Vec::new();

    let mut graph: GraphMap<_, _, Directed> = GraphMap::new();
    for y in 0..=bounds.1 {
        for x in 0..=bounds.0 {
            add(&matrix, &mut graph, bounds, (x,y), (x,y - 1));
            add(&matrix, &mut graph, bounds, (x,y), (x - 1,y));
            add(&matrix, &mut graph, bounds, (x,y), (x,y + 1));
            add(&matrix, &mut graph, bounds, (x,y), (x + 1,y));

            if matrix[y as usize][x as usize] == b'0' {
                starts.push((x, y));
            }
            if matrix[y as usize][x as usize] == b'9' {
                ends.push((x,y));
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        let mut file = File::create("graph.dot").unwrap();
        file.write_all(format!("{:?}", Dot::with_config(&graph, &[])).as_bytes()).unwrap();
        // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        Command::new("dot").arg("-Kneato").arg("-Tsvg").arg("graph.dot").arg("-ooutput.svg").spawn().unwrap();
    }

    let mut part_one = 0;
    let mut part_two = 0;
    for start in starts {
        let mut bfs = Bfs::new(&graph, start);
        while let Some(visited) = bfs.next(&graph) {
            if matrix[visited.1 as usize][visited.0 as usize] == b'9' {
                part_one += 1;
            }
        }

        // every path from every start to every end
        for end in ends.iter() {
            part_two += find_all_paths(&graph, start, end);
        }
    }



    vec![part_one, part_two]
}

#[test]
fn test() {
    assert_eq!(solve("...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9".to_string())[0], 2);

assert_eq!(solve("..90..9
...1.98
...2..7
6543456
765.987
876....
987....".to_string())[0], 4);

assert_eq!(solve("10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01".to_string())[0], 3);

assert_eq!(solve("89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732".to_string())[0], 36);

assert_eq!(solve(".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....".to_string())[1], 3);


    assert_eq!(solve(aoc::input(10))[0], 682);
}
