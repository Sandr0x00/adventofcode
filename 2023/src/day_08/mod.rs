use std::collections::HashMap;

fn traverse(network: &HashMap<&str, (&str, &str)>, instructions: &[char], start: &str, part_two: bool) -> usize {
    let mut i = 0;
    let mut next = start;
    while match part_two {
        true => !next.ends_with('Z'),
        false => next != "ZZZ",
    } {
        let cur = network[next];
        next = match instructions[i % instructions.len()] {
            'L' => cur.0,
            'R' => cur.1,
            _ => unreachable!(),
        };
        i += 1;
    }

    i
}

pub fn solve(input: String) {
    let lines: Vec<_> = input.lines().collect();
    let instructions: Vec<_> = lines[0].chars().collect();

    let mut network = HashMap::new();
    let mut starts = Vec::new();
    for line in lines[2..].iter() {
        let (node, lr) = line.split_once(" = ").unwrap();
        let (l, r) = lr.split_once(", ").unwrap();
        network.insert(node, (&l[1..], &r[..3]));
        if node.ends_with('A') {
            starts.push(node);
        }
    }

    let part_one = traverse(&network, &instructions, "AAA", false);
    let mut steps = Vec::with_capacity(starts.len());
    for start in starts {
        steps.push(traverse(&network, &instructions, start, true));
    }

    aoc::print_solution(&[
        part_one,
        aoc::lcm(&steps),
    ])
}
