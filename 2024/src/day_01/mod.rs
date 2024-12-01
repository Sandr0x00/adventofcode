pub fn solve(input: String) -> Vec<usize> {
    let lines: Vec<_> = input.lines().collect();

    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines.iter() {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<i32>().unwrap());
        right.push(r.parse::<i32>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    let mut diff: usize = 0;
    for i in 0..left.len() {
        diff += (left[i] - right[i]).abs() as usize;
    }

    let mut similarity_score = 0;
    for l in left.iter() {
        similarity_score += *l as usize * right.clone().into_iter().filter(|x| x == l).count();
    }

    vec![diff, similarity_score]
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "3   4
4   3
2   5
1   3
3   9
3   3"
                .to_string()
        ),
        vec![11, 31]
    );
}
