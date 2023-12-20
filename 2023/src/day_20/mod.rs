use std::collections::HashMap;
use std::collections::VecDeque;
use petgraph::Directed;
use petgraph::graphmap::GraphMap;
use petgraph_evcxr::draw_graph;

const DAY: u8 = 20;

const DEBUG: bool = false;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType{
    Broadcast,
    Conjunction(HashMap<String, bool>),
    FlipFlop(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    typ: ModuleType,
    name: String,
    next: Vec<String>,
}

impl Module {
    fn pulse(&mut self, list: &mut VecDeque<(String, String, bool)>, module: &str, input: bool) {
        let forward: Option<bool> = match self.typ {
            ModuleType::Broadcast => Some(input),
            ModuleType::Conjunction(ref mut inputs) => {
                let v = inputs.get_mut(module).unwrap();
                *v = input;
                let mut all_high = true;
                for v in inputs.values_mut() {
                    if !*v {
                        all_high = false;
                    }
                }
                Some(!all_high)
            },
            ModuleType::FlipFlop(ref mut state) => match input {
                // high pulse is ignored and nothing happens
                true => None,
                // low pulse fips on/off
                false => {
                    *state = !*state;
                    Some(*state)
                }
            }
        };


        if let Some(f) = forward {
            for n in self.next.iter() {
                if DEBUG {
                    println!("{} {f} => {n}", self.name);
                }
                list.push_back((n.clone(), self.name.clone(), f));
           }
        }
    }
}

fn button_press(modules: &mut HashMap<String, Module>) -> (bool, (usize, usize)) {
    let mut cnt = (0, 0);
    let mut call_list = VecDeque::new();
    let mut found = false;
    call_list.push_back(("roadcaster".to_owned(), "".to_owned(), false));
    while let Some((i, j, k)) = call_list.pop_front() {
        if (i == "bh" || i == "dl" || i == "vd" || i == "ns") && !k {
            if DEBUG {
                println!("{} {} => {}", j, k, i);
            }
            found = true;
        }
        match k {
            false => cnt.0 += 1,
            true => cnt.1 += 1,
        }
        if let Some(m) = modules.get_mut(&i) {
            m.pulse(&mut call_list, &j, k);
        }
    }

    if DEBUG {
        println!();
        for (k, v) in modules {
            println!("{:?} {:?}", k, v);
        }
        println!();
    }
    (found, cnt)
}

#[allow(dead_code)]
pub fn solve() {
    let input = aoc::input(DAY);

    let mut input_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut modules = HashMap::new();

    if DEBUG {
        // see visualization.png
        let mut graph: GraphMap<&str, &str, Directed> = GraphMap::new();
        for line in input.lines() {
            let (name_raw, next_str) = line.split_once(" -> ").unwrap();
            let next: Vec<_> = next_str.split(", ").collect();
            let name = &name_raw[1..];
            for i in next {
                graph.add_edge(name, i, &name_raw[0..1]);
            }
        }
        draw_graph(&graph);
    }

    // load modules
    for line in input.lines() {
        let (name_raw, next_str) = line.split_once(" -> ").unwrap();
        let next: Vec<_> = next_str.split(", ").map(|s| s.to_owned()).collect();
        let name = name_raw[1..].to_owned();
        let module: Module = Module{name: name.clone(), next: next.clone(), typ: match &name_raw[0..1] {
            "b" => ModuleType::Broadcast,
            "%" => ModuleType::FlipFlop(false),
            "&" => ModuleType::Conjunction(HashMap::<String, bool>::new()),
            _ => unreachable!(),
        }};
        for i in next {
            input_map.entry(i.clone()).and_modify(|v| v.push(name.clone())).or_insert(vec![name.clone()]);
        }
        modules.insert(name, module);
    }

    // update inputs
    for (k, v) in &mut modules {
        if let ModuleType::Conjunction(ref mut inputs) = v.typ {
            for i in input_map[k].iter() {
                inputs.insert(i.to_owned(), false);
            }
        }
    }

    let mut cnt = (0,0);
    let mut vals = [0; 4];
    // split up, see visualization.png
    for (j, n) in ["ls", "bv", "dc", "br"].iter().enumerate() {
        modules.entry("roadcaster".to_owned()).and_modify(|v| v.next = vec![n.to_string()]);
        for i in 0..10000 {
            let (res, add) = button_press(&mut modules);
            if i < 1000 {
                cnt.0 += add.0;
                cnt.1 += add.1;
            }
            if res {
                // +1 for pulse to zh
                vals[j] = i + 1;
                if DEBUG {
                    println!("min {}", i + 1);
                }
                break;
            }
        }
    }

    // we count initial low pulse from broadcaster x4 => 3 times too much
    cnt.0 -= 3000;

    aoc::print_solution(DAY, &[cnt.0 * cnt.1, aoc::lcm(&vals)]);
}
