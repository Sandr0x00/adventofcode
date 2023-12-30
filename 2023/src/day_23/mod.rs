use std::collections::{HashSet, HashMap};
use derivative::*;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

const DEBUG: bool = false;

#[derive(Debug, Clone, PartialEq, Eq, Derivative)]
#[derivative(Hash)]
struct Node {
    id: (usize, usize),
    #[derivative(Hash="ignore")]
    edges: HashMap<(usize, usize), usize>,
}

impl Node {
    fn add_edge(&mut self, dest: (usize, usize), weight: usize) {
        self.edges.insert(dest, weight);
    }

    fn remove_edge(&mut self, dest: (usize, usize)) -> usize {
        self.edges.remove(&dest).unwrap()
    }
}


fn dfs(graph: &HashMap<(usize, usize), Node>, node: (usize, usize), visited: &mut HashSet<(usize, usize)>, dest: (usize, usize), path_len: usize) -> usize {
    visited.insert(node);

    if dest == node {
        return path_len;
    }

    let mut path_lengths = Vec::new();
    for (n, w) in &graph[&node].edges {
        if !visited.contains(n) {
            let l = dfs(graph, *n, &mut visited.clone(), dest, path_len + w);
            path_lengths.push(l);
        }
    }

    match path_lengths.iter().max() {
        Some(m) => *m,
        None => 0,
    }
}

fn hashmap_to_dot(graph: &HashMap<(usize, usize), Node>, name: &str) {
    if !DEBUG {
        return;
    }
    {
        let mut file = File::create(format!("{}.dot", name)).unwrap();
        let _ = file.write("digraph {\n".as_bytes());
        for (id, node) in graph {
            for (t, w) in &node.edges {
                let _ = file.write(format!("    \"{}-{}\" -> \"{}-{}\" [label=\"{}\"]\n", id.0, id.1, t.0, t.1, w).as_bytes());
            }
        }
        let _ = file.write_all("}".as_bytes());
    }
    Command::new("dot").arg("-Kneato").arg("-Tsvg").arg(format!("{}.dot", name)).arg(format!("-o{}.svg", name)).spawn().unwrap();
}

fn positions(matrix: &[Vec<u8>], x: usize, y: usize, bounds: (usize, usize), slopes: bool) -> Vec<(usize, usize)> {
    let mut p = Vec::new();
    if x > 0 && matrix[y][x-1] != b'#' && (!slopes || [b'.', b'<'].contains(&matrix[y][x-1])) {
        p.push((x-1, y))
    }
    if y > 0 && matrix[y-1][x] != b'#' && (!slopes || [b'.', b'^'].contains(&matrix[y-1][x])) {
        p.push((x, y-1))
    }
    if x < bounds.0 && matrix[y][x+1] != b'#' && (!slopes || [b'.', b'>'].contains(&matrix[y][x+1])) {
        p.push((x+1, y))
    }
    if y < bounds.1 && matrix[y+1][x] != b'#' && (!slopes || [b'.', b'v'].contains(&matrix[y+1][x])) {
        p.push((x, y+1))
    }
    p
}

fn collapse(graph: &mut HashMap<(usize, usize), Node>, incoming: &HashMap<(usize, usize), usize>, node: &Node) -> Option<(usize, usize)> {
    if incoming[&node.id] == 2 && node.edges.len() == 2 {
        // we connect 2 nodes, try to directly connect them
        let mut connect_back = true;
        let mut dests = Vec::<(usize, usize)>::new();
        for e in node.edges.keys() {
            // verify we have a connection back
            if !graph[&e].edges.contains_key(&node.id) {
                connect_back = false;
            }
            dests.push(*e);
        }
        // both edges connect back
        if connect_back {
            let w0n = graph.get_mut(&dests[0]).unwrap().remove_edge(node.id);
            let wn1 = graph.get_mut(&node.id).unwrap().remove_edge(dests[1]);
            graph.get_mut(&dests[0]).unwrap().add_edge(dests[1], w0n + wn1);
            let w1n = graph.get_mut(&dests[1]).unwrap().remove_edge(node.id);
            let wn0 = graph.get_mut(&node.id).unwrap().remove_edge(dests[0]);
            graph.get_mut(&dests[1]).unwrap().add_edge(dests[0], w1n + wn0);
            return Some(node.id);
        }
    }

    None
}

fn build_graph(matrix: &[Vec<u8>], bounds: (usize, usize), slopes: bool) -> HashMap<(usize, usize), Node> {
    let mut graph = HashMap::new();
    let mut incoming = HashMap::new();

    for y in 0..bounds.1 + 1 {
        for x in 0..bounds.0 + 1 {
            if matrix[y][x] == b'#' {
                continue;
            }
            let mut node = Node{
                id: (x, y),
                edges: HashMap::new(),
            };
            let possible = positions(matrix, x, y, bounds, slopes);
            for p in possible {
                node.add_edge((p.0, p.1), 1);
                incoming.entry((p.0, p.1)).and_modify(|n| *n += 1).or_insert(1);
            }
            graph.insert((x, y), node);
        }
    }

    // hashmap_to_dot(&graph, if slopes { "g_one_raw" } else { "g_two_raw" });

    // collapse
    let mut last_graph_len = 0;
    while last_graph_len != graph.len() {
        last_graph_len = graph.len();
        let mut removed_nodes = Vec::new();
        for node in graph.clone().values() {
            if let Some(rm) = collapse(&mut graph, &incoming, node) {
                removed_nodes.push(rm);
            }
        }
        for rm in removed_nodes {
            graph.remove(&rm);
        }
    }

    hashmap_to_dot(&graph, if slopes { "g_one_compressed" } else { "g_two_compressed" });

    graph
}

pub fn solve(input: String) {
    let matrix = aoc::parse_matrix(input);
    let bounds = (matrix[0].len() - 1, matrix.len() - 1);

    let graph_one = build_graph(&matrix, bounds, true);
    let graph_two = build_graph(&matrix, bounds, false);

    let start = (1, 0);
    let dest = (bounds.0 - 1, bounds.1);

    aoc::print_solution(&[
        dfs(&graph_one, start, &mut HashSet::new(), dest, 0),
        dfs(&graph_two, start, &mut HashSet::new(), dest, 0)
    ]);
}
