use std::collections::*;
use petgraph::prelude::UnGraphMap;
use petgraph::algo::connected_components;
use petgraph::dot::*;
use petgraph::visit::Dfs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

pub fn solve(input: String) {
    let mut graph = UnGraphMap::new();

    let mut nodes = HashSet::new();

    for line in input.lines() {
        let (n, targets) = line.split_once(": ").unwrap();
        for t in targets.split_ascii_whitespace() {
            if n == "gzr" && t == "qnz" {
                continue;
            }
            if n == "hgk" && t == "pgz" {
                continue;
            }
            if n == "xgs" && t == "lmj" {
                continue;
            }
            graph.add_edge(n, t, format!("{}-{}", n, t));
            nodes.insert(t);
        }
    }

    match connected_components(&graph) {
        1 => {
            // printing seems like a cheese solution, but works, lol
            let mut file = File::create("graph.dot").unwrap();
            file.write_all(format!("{:?}", Dot::with_config(&graph, &[])).as_bytes()).unwrap();
            Command::new("dot").arg("-Kneato").arg("-Tsvg").arg("graph.dot").arg("-ooutput.svg").spawn().unwrap();

            aoc::print_solution(&["=> check output.svg"]);
        },
        2 => {
            let mut dfs = Dfs::new(&graph, "gzr");
            let mut cnt1 = 0;
            while dfs.next(&graph).is_some() {
                cnt1 += 1;
            }
            let cnt2 = nodes.len() - cnt1;

            aoc::print_solution(&[cnt1 * cnt2]);
        },
        _ => unreachable!(),
    }
}
