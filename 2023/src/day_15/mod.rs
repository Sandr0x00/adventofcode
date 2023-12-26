use std::collections::HashMap;



pub fn solve(input: String) {
    let parts = input.split(',');

    let mut boxes = vec![HashMap::<Vec<u8>,(usize,u8)>::new(); 256];
    let mut max_order = vec![0; 256];

    let mut sum_one = 0usize;
    for part in parts {
        let mut full_hash = 0u8;
        let mut label_hash = 0u8;
        let mut label = Vec::new();
        for c in part.as_bytes() {
            match c {
                b'a'..=b'z' => {
                    // label
                    label_hash = label_hash.wrapping_add(*c);
                    label_hash = label_hash.wrapping_mul(17);
                    label.push(*c);
                },
                b'0'..=b'9' => {
                    // data => we expect a = to be before that, but never actually verify that - taking shortcuts here
                    let lens = *c - b'0';

                    // this way we retain the ordering independently of removals since a new one will always be at the end
                    max_order[label_hash as usize] += 1;

                    boxes[label_hash as usize].entry(label.clone()).and_modify(|b| {
                        b.1 = lens
                    }).or_insert((max_order[label_hash as usize], lens));
                },
                b'-' => {
                    // remove lens
                    boxes[label_hash as usize].remove(&label);

                    // we skip re-ordering of the other lenses and do the ordering in the end
                },
                b'\n' => {
                    // ignore new lines
                    continue;
                }
                // skip =
                _ => {},
            }

            full_hash = full_hash.wrapping_add(*c);
            full_hash = full_hash.wrapping_mul(17);
        }
        sum_one += full_hash as usize;
    }

    let mut sum_two = 0;
    for (box_no, box_map) in boxes.iter().enumerate() {
        let mut box_vec: Vec<_> = box_map.iter().map(|(_, data)| data).collect();

        // order according to position
        box_vec.sort_by_key(|x| x.0);

        for (slot, (_, lens)) in box_vec.iter().enumerate() {
            sum_two += (box_no + 1) * (slot + 1) * (*lens as usize);
        }
    }

    aoc::print_solution(&[sum_one, sum_two]);
}
