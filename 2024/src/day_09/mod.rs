use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: String) -> Vec<u64> {
    let chars: Vec<u8> = input.chars().filter_map(|c| c.to_digit(10)).map(|c| c as u8).collect();

    // let mut m = matrix.clone();
    // update_matrix(&mut m, &distinct_antinodes_two);
    // print_matrix(&m);

    let mut id_back: u64 = chars.len() as u64 / 2;
    let mut pos_back = chars.len() - (chars.len() & 1);
    let mut id_front = 0;
    let mut pos_front = 0;
    let mut res: u64 = 0;
    // println!("{}", last_id);

    let mut count_left_back = chars[pos_back];

    let mut cur = 0;
    let mut sum: Vec<u64> = Vec::new();
    for i in (0..chars.len()).step_by(2) {
        // fill from front
        for j in 0..chars[pos_front] {
            sum.push(id_front);
            res += id_front * cur;
            cur += 1;
        }
        id_front += 1;
        pos_front += 1;

        if pos_front == chars.len() {
            break;
        }
        for j in 0..chars[pos_front] {
            sum.push(id_back);
            res += id_back * cur;
            cur += 1;
            count_left_back -= 1;
            // next char from back
            if count_left_back == 0 {
                pos_back -= 2;
                count_left_back = chars[pos_back];
                id_back -= 1;
            }
            if pos_back <= pos_front {
                for j in 0..count_left_back {
                    sum.push(id_back);
                    res += id_back * cur;
                    cur += 1;
                }
                break;
            }
        }
        pos_front += 1;
        if pos_back <= pos_front {
            for j in 0..count_left_back {
                sum.push(id_back);
                res += id_back * cur;
                cur += 1;
            }
            break
        }

    }

    println!("{:?}", sum);

    // for i in 0..chars.len() {
    //     char[i]
    // }

    vec![
        res,
    ]
}

#[test]
fn test() {
    // assert_eq!(solve("12345".to_string()), [1928]);

    // assert_eq!(solve("909090".to_string()), [1928]);

    assert_eq!(solve("2333133121414131402".to_string()), [1928]);

    // assert_eq!(solve(aoc::input(8)), [369, 1169]);
}
