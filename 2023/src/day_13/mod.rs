

const DEBUG: bool = false;

pub fn solve(input: String) {
    let individual_patterns: Vec<_> = input.split("\n\n").collect();

    let mut sum_one = 0;
    let mut sum_two = 0;

    for pattern in individual_patterns {
        let mut matrix = Vec::new();
        let mut x_max = 0;

        for (i, row) in pattern.lines().enumerate() {
            if i == 0 {
                x_max = row.len();
                // add all possible vertical mirror-points
            }
            matrix.push(row.as_bytes());
        }
        let y_max = matrix.len();

        // count mirrored errors,
        // if error is 0 => we found our point of incidence for Part One
        // if error is 1 => we found our point of incidence for Part Two
        let mut mirror_vert = vec![0; x_max - 1];
        let mut mirror_hor = vec![0; y_max - 1];

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
                // search mirror point vertically
                for off in 0..x + 1 {
                    if x + 1 + off == x_max {
                        // reached end
                        break;
                    }
                    if matrix[y][x-off] != matrix[y][x+1+off] {
                        mirror_vert[x] += 1;
                    }
                }

                // search mirror point horizontally
                for off in 0..y + 1 {
                    if y + 1 + off == y_max {
                        // reached end
                        break;
                    }
                    if matrix[y-off][x] != matrix[y+1+off][x] {
                        mirror_hor[y] += 1;
                    }
                }
            }
        }
        if DEBUG {
            println!("VER {:?}", mirror_vert);
            println!("HOR {:?}", mirror_hor);
        }
        for (pos, mirrored) in mirror_vert.iter().enumerate() {
            match mirrored {
                0 => sum_one += pos + 1,
                1 => sum_two += pos + 1,
                _ => {}
            }
        }
        for (pos, mirrored) in mirror_hor.iter().enumerate() {
            match mirrored {
                0 => sum_one += (pos + 1) * 100,
                1 => sum_two += (pos + 1) * 100,
                _ => {}
            }
        }
    }

    aoc::print_solution(&[sum_one, sum_two]);
}
