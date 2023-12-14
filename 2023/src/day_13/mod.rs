use std::collections::HashMap;

const DAY: u8 = 13;
const DEBUG: bool = false;

pub fn solve() {
    let input = aoc::input(DAY);

    let individual_patterns: Vec<_> = input.split("\n\n").collect();

    let mut sum_one = 0;
    let mut sum_two = 0;

    for pattern in individual_patterns {
        // count mirrored errors,
        // if error is 0 => we found our point of incidence for Part One
        // if error is 1 => we found our point of incidence for Part Two
        let mut mirror_vert = HashMap::new();
        let mut mirror_hor = HashMap::new();

        let mut matrix = Vec::new();
        let mut x_max = 0;

        for (i, row) in pattern.lines().enumerate() {
            if i == 0 {
                x_max = row.len();
                // add all possible vertical mirror-points
                for j in 0..row.len() - 1 {
                    mirror_vert.insert(j, 0);
                }
            }
            matrix.push(row.as_bytes());
            mirror_hor.insert(i, 0);
        }
        let y_max = matrix.len();
        mirror_hor.remove(&(matrix.len() - 1));

        if DEBUG {
            println!();
            for m in pattern.lines() {
                println!("{:?}", m);
            }
        }

        // search mirror point vertically
        #[allow(clippy::needless_range_loop)]
        for y in 0..y_max {
            for x in 0..x_max {
                for off in 0..x + 1 {
                    if x + 1 + off == x_max {
                        // reached end
                        break;
                    }
                    if matrix[y][x-off] != matrix[y][x+1+off] {
                        mirror_vert.entry(x).and_modify(|e| *e += 1);
                    }
                }
            }
        }
        if DEBUG {
            println!("VER {:?}", mirror_vert);
        }
        for (pos, mirrored) in mirror_vert {
            if mirrored == 0 {
                sum_one += pos + 1;
            } else if mirrored == 1 {
                sum_two += pos + 1;
            }
        }

        // search mirror point horizontally
        for x in 0..x_max {
            for y in 0..y_max {
                for off in 0..y + 1 {
                    if y + 1 + off == y_max {
                        // reached end
                        break;
                    }
                    if matrix[y-off][x] != matrix[y+1+off][x] {
                        mirror_hor.entry(y).and_modify(|e| *e += 1);
                    }
                }
            }
        }
        if DEBUG {
            println!("HOR {:?}", mirror_hor);
        }
        for (pos, mirrored) in mirror_hor {
            if mirrored == 0 {
                sum_one += (pos + 1) * 100;
            } else if mirrored == 1 {
                sum_two += (pos + 1) * 100;
            }
        }
    }

    aoc::print_solution(DAY, &[sum_one, sum_two]);
}
