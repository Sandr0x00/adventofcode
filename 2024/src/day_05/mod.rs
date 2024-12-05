use std::cmp::Ordering;
use std::collections::HashMap;

fn order_correct(rules: &HashMap<u64, Vec<u64>>, order: &[u64]) -> bool {
    let mut allowed = rules.get(&order[0]).unwrap().clone();
    for page in order[1..].iter() {
        if !allowed.contains(page) {
            return false;
        }
        allowed = match rules.get(page) {
            Some(vec) => vec.clone(),
            None => Vec::<u64>::new(),
        };
    }

    true
}

fn sort(a: &u64, b: &u64, rules: &[(u64, u64)]) -> Ordering {
    if rules.contains(&(*a, *b)) {
        Ordering::Less
    } else if rules.contains(&(*b, *a)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub fn solve(input: String) -> Vec<u64> {
    let (rules_raw, pages) = input.split_once("\n\n").unwrap();

    let mut rules_map = HashMap::new();
    let mut rules_list = Vec::<(u64, u64)>::new();
    for rule in rules_raw.lines() {
        let (first, second) = rule.split_once("|").unwrap();
        let first = first.parse::<u64>().unwrap();
        let second = second.parse::<u64>().unwrap();

        rules_map.entry(first).or_insert_with(Vec::<u64>::new);
        rules_map.get_mut(&first).unwrap().push(second);
        rules_list.push((first, second))
    }

    let mut part_one = 0;
    let mut part_two = 0;
    for order in pages.lines() {
        let mut order = order
            .split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        if order_correct(&rules_map, &order) {
            part_one += order[order.len() / 2];
        } else {
            order.sort_by(|a, b| sort(a, b, &rules_list));
            part_two += order[order.len() / 2];
        }
    }

    vec![part_one, part_two]
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
                .to_string()
        ),
        [143, 123]
    );

    assert_eq!(solve(aoc::input(5)), [5732, 4716]);
}
