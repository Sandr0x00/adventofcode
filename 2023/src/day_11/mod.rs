
const DAY: u8 = 11;

#[derive(Debug, Eq, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

fn traverse(expanded_galaxies: &Vec<Pos>) -> isize {
    let mut sum = 0;
    let mut done = Vec::new();
    for p1 in expanded_galaxies {
        done.push(p1);
        for p2 in expanded_galaxies {
            if done.contains(&p2) {
                continue;
            }
            sum += (p2.x - p1.x).abs() + (p2.y - p1.y).abs();
        }
    }

    sum
}

pub fn solve() {
    let input = aoc::input(DAY);

    let rows: Vec<_> = input.lines().collect();

    let mut empty_cols: Vec<isize> = Vec::new();
    for i in 0..rows[0].len() {
        empty_cols.push(i as isize);
    }
    let mut empty_rows: Vec<isize> = Vec::new();

    let mut galaxies: Vec<_> = Vec::new();

    // expansion
    for (line, y) in rows.iter().zip(0isize..) {
        let row: Vec<_> = line.chars().collect();

        let mut galaxy_in_row = false;
        for (c, x) in row.iter().zip(0isize..) {
            if *c == '#' {
                galaxies.push(Pos{x, y});
                galaxy_in_row = true;
                if empty_cols.contains(&x) {
                    empty_cols.retain(|v| *v != x);
                }
            }
        }
        // if completely empty, expand rows
        if !galaxy_in_row {
            empty_rows.push(y);
        }
    }

    // add expansion to galaxy indices
    let mut expanded_galaxies_one = Vec::new();
    let mut expanded_galaxies_two = Vec::new();
    for Pos{x, y} in &galaxies {
        let x_expansion = empty_cols.iter().filter(|i| i < &x).count() as isize;
        let y_expansion = empty_rows.iter().filter(|i| i < &y).count() as isize;
        expanded_galaxies_one.push(Pos {
            x: x + x_expansion,
            y: y + y_expansion,
        });
        expanded_galaxies_two.push(Pos {
            x: x + 999999 * x_expansion,
            y: y + 999999 * y_expansion,
        });
    }

    let sum_one = traverse(&expanded_galaxies_one);
    let sum_two = traverse(&expanded_galaxies_two);

    aoc::print_solution(DAY, &[sum_one, sum_two])
}