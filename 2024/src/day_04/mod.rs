const TARGET: &[u8] = "XMAS".as_bytes();

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    DownLeft,
    UpRight,
    DownRight,
}

fn rek(
    matrix: &Vec<Vec<u8>>,
    dir: Direction,
    pos: (isize, isize),
    bounds: (isize, isize),
    off: usize,
) -> u64 {
    if off == 4 {
        return 1;
    }

    let next_pos = match dir {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
        Direction::UpLeft => (pos.0 - 1, pos.1 - 1),
        Direction::UpRight => (pos.0 + 1, pos.1 - 1),
        Direction::DownLeft => (pos.0 - 1, pos.1 + 1),
        Direction::DownRight => (pos.0 + 1, pos.1 + 1),
    };

    if next_pos.0 < 0 || next_pos.1 < 0 {
        // we went OOB
        return 0;
    }

    if next_pos.0 > bounds.0 || next_pos.1 > bounds.1 {
        // we went OOB
        return 0;
    }

    if matrix[next_pos.1 as usize][next_pos.0 as usize] == TARGET[off] {
        return rek(matrix, dir, next_pos, bounds, off + 1);
    }

    0
}

fn check_pos(matrix: &[Vec<u8>], pos: (isize, isize), bounds: (isize, isize), char: u8) -> bool {
    if pos.0 < 0 || pos.1 < 0 {
        // we went OOB
        return false;
    }

    if pos.0 > bounds.0 || pos.1 > bounds.1 {
        // we went OOB
        return false;
    }

    matrix[pos.1 as usize][pos.0 as usize] == char
}

const X: u8 = 0x58;
const M: u8 = 0x4d;
const A: u8 = 0x41;
const S: u8 = 0x53;

fn part_two(matrix: &[Vec<u8>], pos: (isize, isize), bounds: (isize, isize)) -> u64 {
    // S S
    //  A
    // M M
    if check_pos(matrix, (pos.0 - 1, pos.1 - 1), bounds, S)
        && check_pos(matrix, (pos.0 + 1, pos.1 - 1), bounds, S)
        && check_pos(matrix, (pos.0 - 1, pos.1 + 1), bounds, M)
        && check_pos(matrix, (pos.0 + 1, pos.1 + 1), bounds, M)
    {
        return 1;
    }

    // M S
    //  A
    // M S
    if check_pos(matrix, (pos.0 - 1, pos.1 - 1), bounds, M)
        && check_pos(matrix, (pos.0 + 1, pos.1 - 1), bounds, S)
        && check_pos(matrix, (pos.0 - 1, pos.1 + 1), bounds, M)
        && check_pos(matrix, (pos.0 + 1, pos.1 + 1), bounds, S)
    {
        return 1;
    }

    // M M
    //  A
    // S S
    if check_pos(matrix, (pos.0 - 1, pos.1 - 1), bounds, M)
        && check_pos(matrix, (pos.0 + 1, pos.1 - 1), bounds, M)
        && check_pos(matrix, (pos.0 - 1, pos.1 + 1), bounds, S)
        && check_pos(matrix, (pos.0 + 1, pos.1 + 1), bounds, S)
    {
        return 1;
    }

    // S M
    //  A
    // S M
    if check_pos(matrix, (pos.0 - 1, pos.1 - 1), bounds, S)
        && check_pos(matrix, (pos.0 + 1, pos.1 - 1), bounds, M)
        && check_pos(matrix, (pos.0 - 1, pos.1 + 1), bounds, S)
        && check_pos(matrix, (pos.0 + 1, pos.1 + 1), bounds, M)
    {
        return 1;
    }

    0
}

pub fn solve(input: String) -> Vec<u64> {
    let matrix = aoc::parse_matrix(input);
    let bounds = (matrix[0].len() as isize - 1, matrix.len() as isize - 1);

    let mut count = 0;
    let mut count2 = 0;
    for y in 0..bounds.1 + 1 {
        for x in 0..bounds.0 + 1 {
            if matrix[y as usize][x as usize] == X {
                count += rek(&matrix, Direction::Up, (x, y), bounds, 1);
                count += rek(&matrix, Direction::Down, (x, y), bounds, 1);
                count += rek(&matrix, Direction::Left, (x, y), bounds, 1);
                count += rek(&matrix, Direction::Right, (x, y), bounds, 1);
                count += rek(&matrix, Direction::UpLeft, (x, y), bounds, 1);
                count += rek(&matrix, Direction::UpRight, (x, y), bounds, 1);
                count += rek(&matrix, Direction::DownLeft, (x, y), bounds, 1);
                count += rek(&matrix, Direction::DownRight, (x, y), bounds, 1);
            }

            if matrix[y as usize][x as usize] == A {
                count2 += part_two(&matrix, (x, y), bounds)
            }
        }
    }

    vec![count, count2]
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
                .to_string()
        ),
        [18, 9]
    );

    // assert_eq!(
    //     solve(
    //         "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()
    //     )[1],
    //     48
    // );
}
