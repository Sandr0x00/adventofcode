use std::collections::HashMap;
use std::collections::HashSet;

#[allow(dead_code)]
fn update_matrix(matrix: &mut [Vec<u8>], positions: &HashSet<(i32, i32)>) {
    for p in positions {
        matrix[p.1 as usize][p.0 as usize] = b'#';
    }
}

fn add_antinode(
    distinct_antinodes: &mut HashSet<(i32, i32)>,
    bounds: &(i32, i32),
    x: i32,
    y: i32,
) -> bool {
    if x < 0 || y < 0 || x > bounds.0 || y > bounds.1 {
        return false;
    }
    distinct_antinodes.insert((x, y));
    true
}

fn create_antinodes(
    matrix: &[Vec<u8>],
    bounds: &(i32, i32),
    distinct_antinodes_one: &mut HashSet<(i32, i32)>,
    distinct_antinodes_two: &mut HashSet<(i32, i32)>,
) {
    let mut existing_nodes: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..=bounds.1 {
        for x in 0..=bounds.0 {
            let elem = matrix[y as usize][x as usize];
            if elem == b'.' {
                continue;
            }

            let nodes = existing_nodes.entry(elem).or_default();
            for n in nodes.iter() {
                let off_x = x - n.0;
                let off_y = y - n.1;

                let mut tmp_x = n.0;
                let mut tmp_y = n.1;
                add_antinode(distinct_antinodes_one, bounds, tmp_x - off_x, tmp_y - off_y);
                while add_antinode(distinct_antinodes_two, bounds, tmp_x, tmp_y) {
                    tmp_x -= off_x;
                    tmp_y -= off_y;
                }

                let mut tmp_x = x;
                let mut tmp_y = y;
                add_antinode(distinct_antinodes_one, bounds, tmp_x + off_x, tmp_y + off_y);
                while add_antinode(distinct_antinodes_two, bounds, tmp_x, tmp_y) {
                    tmp_x += off_x;
                    tmp_y += off_y;
                }
            }
            nodes.push((x, y));
        }
    }
}

pub fn solve(input: String) -> Vec<u64> {
    let (matrix, bounds) = aoc::parse_matrix(input);

    let mut distinct_antinodes_one = HashSet::new();
    let mut distinct_antinodes_two = HashSet::new();
    create_antinodes(
        &matrix,
        &bounds,
        &mut distinct_antinodes_one,
        &mut distinct_antinodes_two,
    );

    // let mut m = matrix.clone();
    // update_matrix(&mut m, &distinct_antinodes_two);
    // print_matrix(&m);

    vec![
        distinct_antinodes_one.len() as u64,
        distinct_antinodes_two.len() as u64,
    ]
}

#[test]
fn test() {
    let m = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."
        .to_string();

    let (matrix, bounds) = aoc::parse_matrix(m);
    let mut distinct_antinodes_one = HashSet::new();

    let mut expected = HashSet::new();
    expected.insert((3, 1));
    expected.insert((6, 7));

    create_antinodes(
        &matrix,
        &bounds,
        &mut distinct_antinodes_one,
        &mut HashSet::new(),
    );
    assert_eq!(distinct_antinodes_one, expected);

    let m = "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."
        .to_string();

    let (matrix, bounds) = aoc::parse_matrix(m);
    let mut distinct_antinodes_one = HashSet::new();

    let mut expected = HashSet::new();
    expected.insert((3, 1));
    expected.insert((6, 7));
    expected.insert((0, 2));
    expected.insert((2, 6));

    create_antinodes(
        &matrix,
        &bounds,
        &mut distinct_antinodes_one,
        &mut HashSet::new(),
    );
    assert_eq!(distinct_antinodes_one, expected);

    assert_eq!(
        solve(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
                .to_string()
        ),
        [14, 34]
    );

    let m = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."
        .to_string();

    let (matrix, bounds) = aoc::parse_matrix(m);
    let mut distinct_antinodes_two = HashSet::new();

    let mut expected = HashSet::new();
    expected.insert((5, 0));
    expected.insert((0, 0));
    expected.insert((3, 1));
    expected.insert((1, 2));
    expected.insert((6, 2));
    expected.insert((9, 3));
    expected.insert((2, 4));
    expected.insert((3, 6));
    expected.insert((4, 8));

    create_antinodes(
        &matrix,
        &bounds,
        &mut HashSet::new(),
        &mut distinct_antinodes_two,
    );
    assert_eq!(distinct_antinodes_two, expected);

    assert_eq!(solve(aoc::input(8)), [369, 1169]);
}
