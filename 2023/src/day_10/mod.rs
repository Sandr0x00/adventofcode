use petgraph::graphmap::{UnGraphMap, GraphMap};
use petgraph::visit::Bfs;
use petgraph::prelude::Undirected;

const DAY: u8 = 10;
const SIZE: usize = 1000;

fn get_xy(num: usize) -> (usize, usize) {
    let y = (num / SIZE) / 2;
    let x = (num % SIZE) / 2;

    (x,y)
}


fn traverse(matrix: &[Vec<char>], x_max: usize, y_max: usize, final_dots: bool) -> (GraphMap<usize, usize, Undirected>, Vec<usize>, usize) {
    let mut dots = Vec::new();

    let mut start = 0;
    let mut graph: GraphMap<_, _, Undirected> = UnGraphMap::new();

    if !final_dots {
        for y in 0..y_max + 1 {
            for x in 0..x_max + 1 {
                let c = matrix[y][x];
                let cur_x = x * 2;
                let cur_y = y * 2;
                dots.push(cur_x + cur_y * SIZE);
                match c {
                    '|' => {
                        if y > 0 && y < y_max {
                            graph.add_edge(cur_x + (cur_y - 1) * SIZE, cur_x + (cur_y    ) * SIZE, 0);
                            graph.add_edge(cur_x + (cur_y    ) * SIZE, cur_x + (cur_y + 1) * SIZE, 0);
                        }
                    },
                    '-' => {
                        if x > 0 && x < x_max {
                            graph.add_edge(cur_x - 1 + cur_y * SIZE, cur_x     + cur_y * SIZE, 0);
                            graph.add_edge(cur_x     + cur_y * SIZE, cur_x + 1 + cur_y * SIZE, 0);
                        }
                    },
                    'L' => {
                        if y > 0 && x < x_max {
                            graph.add_edge(cur_x + (cur_y - 1) * SIZE, cur_x     + cur_y * SIZE, 0);
                            graph.add_edge(cur_x + (cur_y    ) * SIZE, cur_x + 1 + cur_y * SIZE, 0);
                        }
                    },
                    'J' => {
                        if x > 0 && y > 0 {
                            graph.add_edge(cur_x + (cur_y - 1) * SIZE, cur_x     + cur_y * SIZE, 0);
                            graph.add_edge(cur_x + (cur_y    ) * SIZE, cur_x - 1 + cur_y * SIZE, 0);
                        }
                    },
                    '7' => {
                        if x > 0 && y < y_max {
                            graph.add_edge(cur_x + (cur_y    ) * SIZE, cur_x - 1 + cur_y * SIZE, 0);
                            graph.add_edge(cur_x + (cur_y + 1) * SIZE, cur_x     + cur_y * SIZE, 0);
                        }
                    },
                    'F' => {
                        if x < x_max && y < y_max {
                            graph.add_edge(cur_x + (cur_y    ) * SIZE, cur_x + 1 + cur_y * SIZE, 0);
                            graph.add_edge(cur_x + (cur_y + 1) * SIZE, cur_x     + cur_y * SIZE, 0);
                        }
                    },
                    'S' => {
                        // S is connected to all 4 sides
                        if y > 0 {
                            graph.add_edge(cur_x + cur_y * SIZE, cur_x + (cur_y - 1) * SIZE, 0);
                        }
                        if y < y_max {
                            graph.add_edge(cur_x + cur_y * SIZE, cur_x + (cur_y + 1) * SIZE, 0);
                        }
                        if x > 0 {
                            graph.add_edge(cur_x + cur_y * SIZE, cur_x - 1 + cur_y * SIZE, 0);
                        }
                        if x < x_max {
                            graph.add_edge(cur_x + cur_y * SIZE, cur_x + 1 + cur_y * SIZE, 0);
                        }
                        start = cur_x + cur_y * SIZE;
                    },
                    '.' => {},
                    _ => {},
                };
            }
        }
    } else {
        for y in 0..y_max + 1 {
            for x in 0..x_max + 1 {
                let c = matrix[y][x];
                let cur_x = x * 2;
                let cur_y = y * 2;
                if !final_dots {
                    dots.push(cur_x + cur_y * SIZE);
                }
                match c {
                    '|' => {
                        if y > 0 && y < y_max {
                            if x > 0 {
                                graph.add_edge(cur_x - 1 + (cur_y - 1) * SIZE, cur_x - 1 + cur_y       * SIZE, 0);
                                graph.add_edge(cur_x - 1 + cur_y       * SIZE, cur_x - 1 + (cur_y + 1) * SIZE, 0);
                            }
                            if x < x_max {
                                graph.add_edge(cur_x + 1 + (cur_y - 1) * SIZE, cur_x + 1 + cur_y * SIZE, 0);
                                graph.add_edge(cur_x + 1 + cur_y * SIZE, cur_x + 1 + (cur_y + 1) * SIZE, 0);
                            }
                        }
                    },
                    '-' => {
                        if x > 0 && x < x_max {
                            if y > 0 {
                                graph.add_edge(cur_x - 1 + (cur_y - 1) * SIZE, cur_x     + (cur_y - 1) * SIZE, 0);
                                graph.add_edge(cur_x     + (cur_y - 1) * SIZE, cur_x + 1 + (cur_y - 1) * SIZE, 0);
                            }
                            if y < y_max {
                                graph.add_edge(cur_x - 1 + (cur_y + 1) * SIZE, cur_x     + (cur_y + 1) * SIZE, 0);
                                graph.add_edge(cur_x     + (cur_y + 1) * SIZE, cur_x + 1 + (cur_y + 1) * SIZE, 0);
                            }
                        }
                    },
                    'L' => {
                        if y > 0 && x < x_max && y < y_max && x > 0 {
                            // XL
                            graph.add_edge(cur_x - 1 + (cur_y - 1) * SIZE, cur_x - 1 + cur_y       * SIZE, 0);
                            graph.add_edge(cur_x - 1 + cur_y       * SIZE, cur_x - 1 + (cur_y + 1) * SIZE, 0);
                            //  L
                            //  X
                            graph.add_edge(cur_x - 1 + (cur_y + 1) * SIZE, cur_x     + (cur_y + 1) * SIZE, 0);
                            graph.add_edge(cur_x     + (cur_y + 1) * SIZE, cur_x + 1 + (cur_y + 1) * SIZE, 0);
                        }
                    },
                    'J' => {
                        if x > 0 && y > 0 && y < y_max && x < x_max {
                            // JX
                            graph.add_edge(cur_x + 1 + (cur_y - 1) * SIZE, cur_x + 1 + (cur_y    ) * SIZE, 0);
                            graph.add_edge(cur_x + 1 + (cur_y    ) * SIZE, cur_x + 1 + (cur_y + 1) * SIZE, 0);
                            // J
                            // X
                            graph.add_edge(cur_x - 1 + (cur_y + 1) * SIZE, cur_x     + (cur_y + 1) * SIZE, 0);
                            graph.add_edge(cur_x     + (cur_y + 1) * SIZE, cur_x + 1 + (cur_y + 1) * SIZE, 0);
                        }
                    },
                    '7' => {
                        if x > 0 && y < y_max && y > 0 && x < x_max {
                            // 7X
                            graph.add_edge(cur_x + 1 + (cur_y - 1) * SIZE, cur_x + 1 + (cur_y    ) * SIZE, 0);
                            graph.add_edge(cur_x + 1 + (cur_y    ) * SIZE, cur_x + 1 + (cur_y + 1) * SIZE, 0);
                            // X
                            // 7
                            graph.add_edge(cur_x - 1 + (cur_y - 1) * SIZE, cur_x     + (cur_y - 1) * SIZE, 0);
                            graph.add_edge(cur_x     + (cur_y - 1) * SIZE, cur_x + 1 + (cur_y - 1) * SIZE, 0);
                        }
                    },
                    'F' => {
                        if x < x_max && y < y_max && y > 0 && x > 0 {
                            //  X
                            //  F
                            graph.add_edge(cur_x - 1 + (cur_y - 1) * SIZE, cur_x     + (cur_y - 1) * SIZE, 0);
                            graph.add_edge(cur_x     + (cur_y - 1) * SIZE, cur_x + 1 + (cur_y - 1) * SIZE, 0);
                            // XF
                            graph.add_edge(cur_x - 1 + (cur_y - 1) * SIZE, cur_x - 1 + (cur_y    ) * SIZE, 0);
                            graph.add_edge(cur_x - 1 + (cur_y    ) * SIZE, cur_x - 1 + (cur_y + 1) * SIZE, 0);
                        }
                    },
                    'S' => {},
                    '.' => {
                        if y > 0 {
                            graph.add_edge(cur_x + cur_y * SIZE, cur_x + (cur_y - 1) * SIZE, 0);
                        }
                        if y < y_max {
                            graph.add_edge(cur_x + cur_y * SIZE, cur_x + (cur_y + 1) * SIZE, 0);
                        }
                        if x > 0 {
                            graph.add_edge(cur_x + cur_y * SIZE, cur_x - 1 + cur_y * SIZE, 0);
                        }
                        if x < x_max {
                            graph.add_edge(cur_x + cur_y * SIZE, cur_x + 1 + cur_y * SIZE, 0);
                        }
                        dots.push(cur_x + cur_y * SIZE);
                    },
                    _ => {},
                };
            }
        }
    }

    (graph, dots, start)
}

pub fn solve() {
    let input = aoc::input(DAY);

    let rows = input.lines().collect::<Vec<_>>();
    // let y_max = .len() - 1;
    // let x_max = input.lines().collect::<Vec<_>>()[0].len() - 1;
    let y_max = rows.len() - 1;
    let x_max = rows[0].len() - 1;
    let mut matrix = Vec::new();
    for line in rows.iter() {
        matrix.push(line.chars().collect());
    }

    let (graph, mut dots, start) = traverse(&matrix, x_max, y_max, false);

    let mut bfs = Bfs::new(&graph, start);

    let mut dist = 0;
    while let Some(visited) = bfs.next(&graph) {
        dist += 1;
        dots.retain(|&x| x != visited);
    }

    // update "real" dots
    for d in dots {
        let (x, y) = get_xy(d);
        matrix[y][x] = '.';
    }
    let (non_graph, final_dots, _) = traverse(&matrix, x_max, y_max, true);

    let mut inside = Vec::new();
    for d in final_dots {
        bfs = Bfs::new(&non_graph, d);
        let mut outside = false;
        while let Some(visited) = bfs.next(&non_graph) {
            for x in 0..x_max + 1 {
                if visited == (x * 2) {
                    outside = true;
                    break;
                }
                if visited == (x * 2 + y_max * 2 * SIZE) {
                    outside = true;
                    break;
                }
            }
            for y in 0..y_max + 1 {
                if visited == (y * 2 * SIZE) {
                    outside = true;
                    break;
                }
                if visited == (x_max * 2 + y * 2 * SIZE) {
                    outside = true;
                    break;
                }
            }
            if outside {
                break;
            }
        }

        if !outside {
            inside.push(d)
        }
    }

    aoc::print_solution(DAY, &[(dist / 4), inside.len()]);
}