use petgraph::graphmap::{UnGraphMap, GraphMap};
use petgraph::visit::Bfs;
use petgraph::prelude::Undirected;
// use petgraph_evcxr::draw_graph;
use std::collections::HashSet;

const DAY: u8 = 10;
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


fn traverse(matrix: &[Vec<char>], max: (isize, isize), final_dots: bool) -> (GraphMap<usize, usize, Undirected>, HashSet<usize>, usize) {
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
                    '|' => {
                        add(&mut graph, max, cur, (cur_x,     cur_y - 1));
                        add(&mut graph, max, cur, (cur_x,     cur_y + 1));
                    },
                    '-' => {
                        add(&mut graph, max, cur, (cur_x - 1, cur_y    ));
                        add(&mut graph, max, cur, (cur_x + 1, cur_y    ));
                    },
                    'L' => {
                        add(&mut graph, max, cur, (cur_x,     cur_y - 1));
                        add(&mut graph, max, cur, (cur_x + 1, cur_y    ));
                    },
                    'J' => {
                        add(&mut graph, max, cur, (cur_x,     cur_y - 1));
                        add(&mut graph, max, cur, (cur_x - 1, cur_y    ));
                    },
                    '7' => {
                        add(&mut graph, max, cur, (cur_x - 1, cur_y    ));
                        add(&mut graph, max, cur, (cur_x,     cur_y + 1));
                    },
                    'F' => {
                        add(&mut graph, max, cur, (cur_x + 1, cur_y    ));
                        add(&mut graph, max, cur, (cur_x,     cur_y + 1));
                    },
                    'S' => {
                        // S is connected to all 4 sides
                        add(&mut graph, max, cur, (cur_x,     cur_y - 1));
                        add(&mut graph, max, cur, (cur_x,     cur_y + 1));
                        add(&mut graph, max, cur, (cur_x - 1, cur_y    ));
                        add(&mut graph, max, cur, (cur_x + 1, cur_y    ));
                        start = cur_x as usize + cur_y as usize * SIZE;
                    },
                    '.' => {},
                    _ => {},
                };
            } else {
                match c {
                    '|' => {
                        // X X
                        // X|X
                        // X X
                        add(&mut graph, max, (cur_x - 1, cur_y - 1), (cur_x - 1, cur_y    ));
                        add(&mut graph, max, (cur_x - 1, cur_y    ), (cur_x - 1, cur_y + 1));
                        add(&mut graph, max, (cur_x + 1, cur_y - 1), (cur_x + 1, cur_y    ));
                        add(&mut graph, max, (cur_x + 1, cur_y    ), (cur_x + 1, cur_y + 1));
                    },
                    '-' => {
                        // XXX
                        //  -
                        // XXX
                        add(&mut graph, max, (cur_x - 1, cur_y - 1), (cur_x,     cur_y - 1));
                        add(&mut graph, max, (cur_x,     cur_y - 1), (cur_x + 1, cur_y - 1));
                        add(&mut graph, max, (cur_x - 1, cur_y + 1), (cur_x,     cur_y + 1));
                        add(&mut graph, max, (cur_x,     cur_y + 1), (cur_x + 1, cur_y + 1));
                    },
                    'L' => {
                        // XL
                        add(&mut graph, max, (cur_x - 1, cur_y - 1), (cur_x - 1, cur_y    ));
                        add(&mut graph, max, (cur_x - 1, cur_y    ), (cur_x - 1, cur_y + 1));
                        //  L
                        //  X
                        add(&mut graph, max, (cur_x - 1, cur_y + 1), (cur_x,     cur_y + 1));
                        add(&mut graph, max, (cur_x,     cur_y + 1), (cur_x + 1, cur_y + 1));
                    },
                    'J' => {
                        // JX
                        add(&mut graph, max, (cur_x + 1, cur_y - 1), (cur_x + 1, cur_y    ));
                        add(&mut graph, max, (cur_x + 1, cur_y    ), (cur_x + 1, cur_y + 1));
                        // J
                        // X
                        add(&mut graph, max, (cur_x - 1, cur_y + 1), (cur_x,     cur_y + 1));
                        add(&mut graph, max, (cur_x,     cur_y + 1), (cur_x + 1, cur_y + 1));
                    },
                    '7' => {
                        // 7X
                        add(&mut graph, max, (cur_x + 1, cur_y - 1), (cur_x + 1, cur_y    ));
                        add(&mut graph, max, (cur_x + 1, cur_y    ), (cur_x + 1, cur_y + 1));
                        // X
                        // 7
                        add(&mut graph, max, (cur_x - 1, cur_y - 1), (cur_x,     cur_y - 1));
                        add(&mut graph, max, (cur_x,     cur_y - 1), (cur_x + 1, cur_y - 1));
                    },
                    'F' => {
                        //  X
                        //  F
                        add(&mut graph, max, (cur_x - 1, cur_y - 1), (cur_x,     cur_y - 1));
                        add(&mut graph, max, (cur_x    , cur_y - 1), (cur_x + 1, cur_y - 1));
                        // XF
                        add(&mut graph, max, (cur_x - 1, cur_y - 1), (cur_x - 1, cur_y    ));
                        add(&mut graph, max, (cur_x - 1, cur_y    ), (cur_x - 1, cur_y + 1));
                    },
                    'S' => {},
                    '.' => {
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

#[allow(dead_code)]
pub fn solve() {
    let input = aoc::input(DAY);

    let rows = input.lines().collect::<Vec<_>>();
    let max = (
        (rows.len() - 1) as isize,
        (rows[0].len() - 1) as isize,
    );


    let mut matrix = Vec::new();
    for line in rows.iter() {
        matrix.push(line.chars().collect());
    }

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
        matrix[y][x] = '.';
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

    aoc::print_solution(DAY, &[(dist / 4), inside]);
}