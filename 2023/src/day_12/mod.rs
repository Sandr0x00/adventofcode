use regex::Regex;
#[allow(unused_imports)]
use std::thread;
use std::time::Instant;
use std::str::from_utf8_unchecked;

fn rek(re: &Regex, cur: &mut Vec<u8>, i: usize, count: &mut usize) {
    if i == cur.len() {
        // reached end
        *count += 1;
        return;
    }
    if cur[i] == b'?' {
        cur[i] = b'.';
        unsafe {
            if re.is_match(from_utf8_unchecked(cur)) {
                // println!(". {:?}", cur);
                rek(re, &mut cur.clone(), i + 1, count);
            }
        }
        cur[i] = b'#';
        unsafe {
            if re.is_match(from_utf8_unchecked(cur)) {
                // println!("# {:?}", cur);
                rek(re, &mut cur.clone(), i + 1, count);
            }
        }
    } else {
        rek(re, cur, i + 1, count);
    }
}

fn create_re(record: &str) -> String {
    format!(r"^[\.\?]*{}[\.\?]*$", record.split(',').map(|s| format!(r"[#\?]{{{s}}}")).collect::<Vec<_>>().join(r"[\.\?]+"))
}

pub fn solve(input: String) {
    let start_time = Instant::now();

    let mut arrangements_one = 0;
    // let mut threads = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let (springs, record) = line.split_once(' ').unwrap();

        let unfolded_springs = [springs; 5].join("?");
        let unfolded_record = [record; 5].join(",");

        // build regex 1
        let mut count_one = 0;
        let re_one = Regex::new(&create_re(record)).unwrap();
        rek(&re_one, &mut springs.as_bytes().to_vec(), 0, &mut count_one);
        arrangements_one += count_one;

        // build regex 2
        // let re_two = Regex::new(&create_re(&unfolded_record)).unwrap();
        // threads.push(thread::spawn(move || {
        //     let mut count_two = 0;
        //     rek(&re_two, &mut unfolded_springs.as_bytes().to_vec(), 0, &mut count_two);
        //     println!("{i} - {count_two} - {:?}", start_time.elapsed());
        //     count_two
        // }));
    }
    println!("runtime: {:?}", start_time.elapsed());

    let mut arrangements_two = 0;
    // for t in threads {
    //     arrangements_two += t.join().unwrap();
    //     io::stdout().flush().unwrap();
    // }
    // println!("{:?}", start_time.elapsed());


    aoc::print_solution(&[arrangements_one, arrangements_two]);
}

// def solve_puzzle(input_str, counts):
//     result = [None] * len(input_str)
//     return backtrack(input_str, counts, result, 0)

// def backtrack(input_str, counts, result, index):
//     if index == len(input_str):
//         return counts == [result.count('#')]

//     if input_str[index] == '.':
//         result[index] = '.'
//         if backtrack(input_str, counts, result, index + 1):
//             return result

//         result[index] = '#'
//         if backtrack(input_str, counts, result, index + 1):
//             return result

//     elif input_str[index] == '#':
//         result[index] = '#'
//         if backtrack(input_str, counts, result, index + 1):
//             return result

//     result[index] = '.'
//     if backtrack(input_str, counts, result, index + 1):
//         return result

//     return None
