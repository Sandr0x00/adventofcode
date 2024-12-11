use std::collections::HashMap;
use std::collections::HashSet;

fn part_one(chars: &Vec<u8>) -> u64 {
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
            break;
        }
    }

    println!("{:?}", sum);

    res
}

#[derive(Debug, Clone)]
struct Block {
    id: i32,
    sz: u8,
}

fn move_element(vec: &mut Vec<i32>, from: usize, to: usize) {
    if from >= vec.len() || to >= vec.len() {
        return; // Ensure the indices are valid
    }

    let elem = vec.remove(from); // Remove the element from the original position
    vec.insert(to, elem); // Insert it at the new position
}

fn part_two(chars: &Vec<u8>) -> u64 {
    let mut id_back: u64 = chars.len() as u64 / 2;
    let mut pos_back = chars.len() - (chars.len() & 1);
    let mut id_front = 0;
    let mut pos_front = 0;
    let mut res: u64 = 0;
    // println!("{}", last_id);

    let mut disk = [0; 4096];

    let mut count_left_back = chars[pos_back];

    let mut cur = 0;
    let mut sum: Vec<u64> = Vec::new();

    let mut id = 0;
    let mut disk2 = Vec::<Block>::new();
    let mut blocks = HashMap::new();
    for i in 0..chars.len() {
        if i & 1 == 0 {
            let block = Block {
                id: id,
                sz: chars[i],
            };
            blocks.insert(id, block.clone());
            disk2.push(block);
            id += 1;
        } else {
            disk2.push(Block {
                id: -1,
                sz: chars[i],
            });
        }
        // fill from front
        // for j in 0..chars[pos_front] {
        //     disk[cur] = id;
        //     cur += 1;
        // }
        // pos_front += 1;

        // pos_front += 1;

        // if pos_front == chars.len() {
        //     break;
        // }

        // for j in 0..chars[pos_front] {
        //     disk[cur] = -1;
        //     cur += 1;
        // }
        // // next block from back
        // pos_front += 1;
        // if pos_back <= pos_front {
        //     // for j in 0..count_left_back {
        //     //     sum.push(id_back);
        //     //     res += id_back * cur;
        //     //     cur += 1;
        //     // }
        //     break;
        // }
    }

    let mut disk3 = disk2.clone();
    disk3.reverse();

    // try to move every block exactly once

    for id in (1..=id_back).rev() {
        let src = blocks.get(&(id as i32)).unwrap();
        let mut mv_pos = 0;
        let mut leftover = Block { id: -1, sz: 0 };
        println!("id {id}");

        for i in 0..disk2.len() {
            let target = &disk2[i];
            if target.id == id as i32 {
                // don't overmove
                break;
            }
            if target.id == -1 && target.sz >= src.sz {
                mv_pos = i;
                leftover = Block {
                    id: -1,
                    sz: target.sz - src.sz,
                };
                break;
            }
        }

        println!("mv {mv_pos}");

        if mv_pos > 0 {
            // remove old block
            disk2 = disk2
                .into_iter()
                .map(|x| {
                    if x.id == id as i32 {
                        Block { id: -1, sz: x.sz }
                    } else {
                        x
                    }
                })
                .collect();

            // remove empty block
            disk2.remove(mv_pos);

            // add new block + leftover
            disk2.insert(mv_pos, src.clone());
            if leftover.sz > 0 {
                disk2.insert(mv_pos + 1, leftover.clone());
            }
        }
    }

    // cleanup
    // let mut id = chars.len() as u64 / 2;
    // for i in (chars.len() - 1..=0).step_by(2) {

    // }
    let mut res: u64 = 0;
    let mut pos = 0;
    for b in disk2.iter() {
        for i in 0..b.sz {
            if b.id > 0 {
                res += (pos * b.id) as u64;
            }
            pos += 1;
        }
    }

    println!("{:?}", disk2);

    res
}

pub fn solve(input: String) -> Vec<u64> {
    let chars: Vec<u8> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as u8)
        .collect();

    vec![part_one(&chars), part_two(&chars)]
}

#[test]
fn test() {
    assert_eq!(solve("2333133121414131402".to_string()), [1928, 2858]);

    assert_eq!(solve(aoc::input(9)), [6330095022244, 6359491814941]);
}
