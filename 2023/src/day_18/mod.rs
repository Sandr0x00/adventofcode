use itertools::Itertools;

const NAIVE: bool = true;

fn flood_fill(grid: &mut Vec<Vec<u8>>, x: usize, y: usize, count: &mut usize) {
    if grid[y][x] == b'.' {
        *count += 1;
        grid[y][x] = b'#';
        flood_fill(grid, x-1, y, count);
        flood_fill(grid, x+1, y, count);
        flood_fill(grid, x, y-1, count);
        flood_fill(grid, x, y+1, count);
    }
}

fn calc(ins: Vec<(char, isize)>) -> isize{
    let (mut x, mut y) = (0, 0);
    let mut points: Vec<(isize, isize)> = vec![(x,y)];
    let mut area = 0;
    for (d, l) in ins {
        match d {
            'L' => x -= l,
            'R' => x += l,
            'U' => y -= l,
            'D' => y += l,
            _ => unreachable!(),
        }
        points.push((x,y));
        area += l;
    }
    area /= 2;
    let enclosed: isize = points.iter().tuple_windows().map(|((x1,y1), (x2,y2))| (x2+x1) * (y2-y1)).sum();
    area += enclosed / 2 + 1;

    area
}

pub fn solve(input: String) {
    let mut instructions = Vec::new();
    let mut instructions2 = Vec::new();

    // NAIVE
    let mut pos = (500, 500);
    let mut grid = vec![vec![b'.'; 1000]; 1000];
    let mut count = 0;

    for line in input.lines() {
        let dir = line.chars().collect::<Vec<_>>()[0];
        let (length, ins2) = line[2..].split_once(' ').unwrap();
        let len = length.parse::<isize>().unwrap();
        instructions.push((dir, len));

        if NAIVE {
            for _ in 0..len {
                match dir {
                    'R' => pos.0 += 1,
                    'D' => pos.1 += 1,
                    'L' => pos.0 -= 1,
                    'U' => pos.1 -= 1,
                    _ => {},
                };
                grid[pos.1][pos.0] = b'#';
                count += 1;
            }
        }

        let len2 = isize::from_str_radix(&ins2[2..7], 16).unwrap();
        let dir2 = match &ins2[7..8] {
            "0" => 'R',
            "1" => 'D',
            "2" => 'L',
            "3" => 'U',
            _ => unreachable!(),
        };
        instructions2.push((dir2, len2));
    }

    if NAIVE {
        aoc::print_matrix(&grid);
        flood_fill(&mut grid, 499, 499, &mut count);
        aoc::print_matrix(&grid);
        println!("Naive Solution for One {}", count);
    }

    aoc::print_solution(&[
        calc(instructions),
        calc(instructions2)
    ]);
}
