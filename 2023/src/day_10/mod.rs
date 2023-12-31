use petgraph::graphmap::{UnGraphMap, GraphMap};
use petgraph::visit::Bfs;
use petgraph::prelude::Undirected;
// use petgraph_evcxr::draw_graph;
use std::collections::HashSet;

const SIZE: usize = 1000;

fn get_xy(num: usize) -> (usize, usize) {
    let y = (num / SIZE) / 2;
    let x = (num % SIZE) / 2;

    (x,y)
}

fn add(g: &mut GraphMap<usize, usize, Undirected>, max: (isize, isize), p1: (isize, isize), p2: (isize, isize)) {
    if p1.0 < 0 || p1.1 < 0 || p1.0 > max.0 * 2 || p1.1 > max.1 * 2 {
        return;
    }
    if p2.0 < 0 || p2.1 < 0 || p2.0 > max.0 * 2 || p2.1 > max.1 * 2 {
        return;
    }

    g.add_edge((p1.0 + p1.1 * SIZE as isize) as usize, (p2.0 + p2.1 * SIZE as isize) as usize, 0);
}

fn traverse(matrix: &[Vec<u8>], max: (isize, isize), final_dots: bool) -> (GraphMap<usize, usize, Undirected>, HashSet<usize>, usize) {
    let mut dots = HashSet::new();

    let mut start = 0;
    let mut graph: GraphMap<_, _, Undirected> = UnGraphMap::new();

    for y in 0..max.1 + 1 {
        for x in 0..max.0 + 1 {
            let c = matrix[y as usize][x as usize];
            let cur_x = x * 2;
            let cur_y = y * 2;

            if !final_dots {
                let cur = (cur_x, cur_y);
                dots.insert(cur_x as usize + cur_y as usize * SIZE);
                match c {
                    b'|' => {
                        add(&mut graph, max, cur, (cur_x,     cur_y - 1));
                        add(&mut graph, max, cur, (cur_x,     cur_y + 1));
                    },
                    b'-' => {
                        add(&mut graph, max, cur, (cur_x - 1, cur_y    ));
                        add(&mut graph, max, cur, (cur_x + 1, cur_y    ));
                    },
                    b'L' => {
                        add(&mut graph, max, cur, (cur_x,     cur_y - 1));
                        add(&mut graph, max, cur, (cur_x + 1, cur_y    ));
                    },
                    b'J' => {
                        add(&mut graph, max, cur, (cur_x,     cur_y - 1));
                        add(&mut graph, max, cur, (cur_x - 1, cur_y    ));
                    },
                    b'7' => {
                        add(&mut graph, max, cur, (cur_x - 1, cur_y    ));
                        add(&mut graph, max, cur, (cur_x,     cur_y + 1));
                    },
                    b'F' => {
                        add(&mut graph, max, cur, (cur_x + 1, cur_y    ));
                        add(&mut graph, max, cur, (cur_x,     cur_y + 1));
                    },
                    b'S' => {
                        // S is connected to all 4 sides
                        add(&mut graph, max, cur, (cur_x,     cur_y - 1));
                        add(&mut graph, max, cur, (cur_x,     cur_y + 1));
                        add(&mut graph, max, cur, (cur_x - 1, cur_y    ));
                        add(&mut graph, max, cur, (cur_x + 1, cur_y    ));
                        start = cur_x as usize + cur_y as usize * SIZE;
                    },
                    b'.' => {},
                    _ => {},
                };
            } else {
                match c {
                    b'|' => {
                        // X X
                        // X|X
                        // X X
                        add(&mut graph, max, (cur_x - 1, cur_y - 1), (cur_x - 1, cur_y    ));
                        add(&mut graph, max, (cur_x - 1, cur_y    ), (cur_x - 1, cur_y + 1));
                        add(&mut graph, max, (cur_x + 1, cur_y - 1), (cur_x + 1, cur_y    ));
                        add(&mut graph, max, (cur_x + 1, cur_y    ), (cur_x + 1, cur_y + 1));
                    },
                    b'-' => {
                        // XXX
                        //  -
                        // XXX
                        add(&mut graph, max, (cur_x - 1, cur_y - 1), (cur_x,     cur_y - 1));
                        add(&mut graph, max, (cur_x,     cur_y - 1), (cur_x + 1, cur_y - 1));
                        add(&mut graph, max, (cur_x - 1, cur_y + 1), (cur_x,     cur_y + 1));
                        add(&mut graph, max, (cur_x,     cur_y + 1), (cur_x + 1, cur_y + 1));
                    },
                    b'L' => {
                        // XL
                        add(&mut graph, max, (cur_x - 1, cur_y - 1), (cur_x - 1, cur_y    ));
                        add(&mut graph, max, (cur_x - 1, cur_y    ), (cur_x - 1, cur_y + 1));
                        //  L
                        //  X
                        add(&mut graph, max, (cur_x - 1, cur_y + 1), (cur_x,     cur_y + 1));
                        add(&mut graph, max, (cur_x,     cur_y + 1), (cur_x + 1, cur_y + 1));
                    },
                    b'J' => {
                        // JX
                        add(&mut graph, max, (cur_x + 1, cur_y - 1), (cur_x + 1, cur_y    ));
                        add(&mut graph, max, (cur_x + 1, cur_y    ), (cur_x + 1, cur_y + 1));
                        // J
                        // X
                        add(&mut graph, max, (cur_x - 1, cur_y + 1), (cur_x,     cur_y + 1));
                        add(&mut graph, max, (cur_x,     cur_y + 1), (cur_x + 1, cur_y + 1));
                    },
                    b'7' => {
                        // 7X
                        add(&mut graph, max, (cur_x + 1, cur_y - 1), (cur_x + 1, cur_y    ));
                        add(&mut graph, max, (cur_x + 1, cur_y    ), (cur_x + 1, cur_y + 1));
                        // X
                        // 7
                        add(&mut graph, max, (cur_x - 1, cur_y - 1), (cur_x,     cur_y - 1));
                        add(&mut graph, max, (cur_x,     cur_y - 1), (cur_x + 1, cur_y - 1));
                    },
                    b'F' => {
                        //  X
                        //  F
                        add(&mut graph, max, (cur_x - 1, cur_y - 1), (cur_x,     cur_y - 1));
                        add(&mut graph, max, (cur_x    , cur_y - 1), (cur_x + 1, cur_y - 1));
                        // XF
                        add(&mut graph, max, (cur_x - 1, cur_y - 1), (cur_x - 1, cur_y    ));
                        add(&mut graph, max, (cur_x - 1, cur_y    ), (cur_x - 1, cur_y + 1));
                    },
                    b'S' => {},
                    b'.' => {
                        add(&mut graph, max, (cur_x,     cur_y    ), (cur_x,     cur_y - 1));
                        add(&mut graph, max, (cur_x,     cur_y    ), (cur_x,     cur_y + 1));
                        add(&mut graph, max, (cur_x,     cur_y    ), (cur_x - 1, cur_y    ));
                        add(&mut graph, max, (cur_x,     cur_y    ), (cur_x + 1, cur_y    ));
                        dots.insert(cur_x as usize + cur_y as usize * SIZE);
                    },
                    _ => {},
                };
            }
        }
    }

    (graph, dots, start)
}

pub fn solve(input: String) {
    let mut matrix = aoc::parse_matrix(input);
    let max = (matrix[0].len() as isize - 1, matrix.len() as isize - 1);

    let (graph, mut dots, start) = traverse(&matrix, max, false);

    // draw_graph(&graph);

    let mut bfs = Bfs::new(&graph, start);

    let mut dist = 0;
    while let Some(visited) = bfs.next(&graph) {
        dist += 1;
        dots.remove(&visited);
    }

    // update "real" dots
    for d in dots {
        let (x, y) = get_xy(d);
        matrix[y][x] = b'.';
    }
    let (non_graph, final_dots, _) = traverse(&matrix, max, true);

    let mut known_outside = HashSet::new();

    for x in 0..max.0 + 1 {
        known_outside.insert((x * 2) as usize);
        known_outside.insert((x * 2 + max.1 * 2 * SIZE as isize) as usize);
    }

    for y in 0..max.1 + 1 {
        known_outside.insert((y * 2 * SIZE as isize) as usize);
        known_outside.insert((max.0 * 2 + y * 2 * SIZE as isize) as usize);
    }


    // this takes quite long (10s), maybe future improvement
    let mut inside = 0;
    for d in final_dots {
        if known_outside.contains(&d) {
            // trivial outside
            continue;
        }

        let mut search = Bfs::new(&non_graph, d);
        let mut outside = false;
        while let Some(visited) = search.next(&non_graph) {
            if known_outside.contains(&visited) {
                outside = true;
                break;
            }
        }

        if !outside {
            inside += 1;
        }
    }

    aoc::print_solution(&[(dist / 4), inside]);
}