use std::collections::HashMap;

fn load(matrix: &mut [Vec<u8>], max: (usize, usize)) -> usize{
    let mut sum = 0;
    for (y, row) in matrix.iter().enumerate() {
        for c in row {
            if *c == b'O' {
                sum += max.1 - y;
            }
        }
    }

    sum
}

fn tilt_north(matrix: &mut [Vec<u8>], max: (usize, usize)) {
    let mut off_list = vec![0; max.0];

    for y in 0..max.1 {
        for x in 0..max.0 {
            if matrix[y][x] == b'O' {
                // swap
                matrix[y][x] = b'.';
                matrix[off_list[x]][x] = b'O';

                off_list[x] += 1;
            }
            if matrix[y][x] == b'#' {
                off_list[x] = y + 1;
            }
        }
    }
}

fn tilt_west(matrix: &mut [Vec<u8>], max: (usize, usize)) {
    let mut off_list = vec![0; max.1];

    for y in 0..max.1 {
        for x in 0..max.0 {
            if matrix[y][x] == b'O' {
                // swap
                matrix[y][x] = b'.';
                matrix[y][off_list[y]] = b'O';

                off_list[y] += 1;
            }
            if matrix[y][x] == b'#' {
                off_list[y] = x + 1;
            }
        }
    }
}

fn tilt_south(matrix: &mut [Vec<u8>], max: (usize, usize)) {
    let mut off_list = vec![max.1 - 1; max.0];

    for y in (0..max.1).rev() {
        for x in 0..max.0 {
            if matrix[y][x] == b'O' {
                // swap
                matrix[y][x] = b'.';
                matrix[off_list[x]][x] = b'O';

                if off_list[x] > 0 {
                    off_list[x] -= 1;
                }
            }
            if matrix[y][x] == b'#' && y > 0 {
                off_list[x] = y - 1;
            }
        }
    }
}

fn tilt_east(matrix: &mut [Vec<u8>], max: (usize, usize)) {
    let mut off_list = vec![max.0 - 1; max.1];

    for y in 0..max.1 {
        for x in (0..max.0).rev() {
            if matrix[y][x] == b'O' {
                // swap
                matrix[y][x] = b'.';
                matrix[y][off_list[y]] = b'O';

                if off_list[y] > 0 {
                    off_list[y] -= 1;
                }
            }
            if matrix[y][x] == b'#' && x > 0 {
                off_list[y] = x - 1;
            }
        }
    }
}

fn pretty(matrix: &[Vec<u8>]) {
    println!();
    for row in matrix {
        for c in row {
            print!("{}", *c as char);
        }
        println!()
    }
}

pub fn solve(input: String) {
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    let mut max = (0, 0);

    for line in input.lines() {
        max.0 = line.len();
        matrix.push(line.as_bytes().to_vec());
    }
    max.1 = matrix.len();

    let mut load_one = 0;

    let mut memo = HashMap::new();
    let mut cycle = 0;
    let max_cycle = 1000000000;
    while cycle < max_cycle {
        tilt_north(&mut matrix, max);
        if cycle == 0 {
            load_one = load(&mut matrix, max);
        }
        // pretty(&matrix);

        tilt_west(&mut matrix, max);
        // pretty(&matrix);

        tilt_south(&mut matrix, max);
        // pretty(&matrix);

        tilt_east(&mut matrix, max);
        // pretty(&matrix);

        if let Some(last_seen) = memo.insert(matrix.clone(), cycle) {
            // fast forward (if possible)
            let cycle_len = cycle - last_seen;
            // 9  +=     7     * ((   100    - 1 -   9  ) /     7    )
            // 9  += 7 * 12 => 93
            // not the perfect solution, but easy enough
            cycle += cycle_len * ((max_cycle - 1 - cycle) / cycle_len);
        }
        cycle += 1;
    }

    let load_two = load(&mut matrix, max);

    aoc::print_solution(&[load_one, load_two]);
}
