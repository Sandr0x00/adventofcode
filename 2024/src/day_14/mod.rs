use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use once_cell::sync::Lazy;

const BOUNDS: (i32, i32) = if cfg!(test) { (11, 7) } else { (101, 103) };

static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"p=(?P<x>\d+),(?P<y>\d+) v=(?P<vx>-?\d+),(?P<vy>-?\d+)").unwrap()
});

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn new(line: &str) -> Robot {
        let caps = RE.captures(line).unwrap();

        Robot {
            x: caps["x"].parse().unwrap(),
            y: caps["y"].parse().unwrap(),
            vx: caps["vx"].parse().unwrap(),
            vy: caps["vy"].parse().unwrap(),
        }
    }

    fn mv(&mut self, steps: i32) {
        self.x += self.vx * steps;
        self.y += self.vy * steps;

        // wrap
        self.x %= BOUNDS.0;
        self.y %= BOUNDS.1;

        // get back into positive range
        self.x += BOUNDS.0;
        self.y += BOUNDS.1;

        // and make sure we're still in bounds
        self.x %= BOUNDS.0;
        self.y %= BOUNDS.1;
    }

    fn try_mv(&self, steps: i32) -> (i32, i32) {
        let mut x = self.x + self.vx * steps;
        let mut y = self.y + self.vy * steps;

        // wrap
        x %= BOUNDS.0;
        y %= BOUNDS.1;

        // get back into positive range
        x += BOUNDS.0;
        y += BOUNDS.1;

        // and make sure we're still in bounds
        x %= BOUNDS.0;
        y %= BOUNDS.1;

        (x, y)
    }
}

fn draw_image(robots: &Vec<Robot>, steps: i32) {
    let mut map = [[0; BOUNDS.0 as usize]; BOUNDS.1 as usize];

    for r in robots {
        let pos = r.try_mv(steps);
        map[pos.1 as usize][pos.0 as usize] += 1;
    }

    let mut file = File::create(format!("src/day_14/arrangement_{}.txt", steps + 100)).unwrap();

    for row in map {
        for cell in row {
            if cell == 0 {
                file.write_all(b".").unwrap();
            } else {
                file.write_all(b"*").unwrap();
            }
        }
        file.write_all(b"\n").unwrap();
    }
}

pub fn solve(input: String) -> Vec<u64> {
    let lines: Vec<_> = input.lines().collect();

    let mut robots = Vec::new();
    for line in lines.iter() {
        robots.push(Robot::new(line));
    }

    for r in robots.iter_mut() {
        r.mv(100);
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for r in &robots {
        if r.x == BOUNDS.0 / 2 {
            // center
            continue;
        }
        if r.y == BOUNDS.1 / 2 {
            // center
            continue;
        }
        let left = r.x < BOUNDS.0 / 2;
        let top = r.y < BOUNDS.1 / 2;
        if left && top {
            q1 += 1;
        } else if left && !top {
            q3 += 1;
        } else if !left && top {
            q2 += 1;
        } else if !left && !top {
            q4 += 1;
        }
    }

    let mut steps = 1;
    loop {
        // no overlaps, basically got lucky this worked
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        let mut ok = true;
        for r in &robots {
            let pos = r.try_mv(steps);
            if set.contains(&pos) {
                ok = false;
                break;
            }
            set.insert(pos);
        }
        if ok {
            if cfg!(debug_assertions) {
                draw_image(&robots, steps);
            }
            break;
        }
        steps += 1;
    }

    // + 100 since we already moved 100 for part one
    vec![q1 * q2 * q3 * q4, 100 + steps as u64]
}

#[test]
fn test() {
    let mut r = Robot::new("p=2,4 v=2,-3");
    assert_eq!(r.x, 2);
    assert_eq!(r.y, 4);
    r.mv(5);
    assert_eq!(r.x, 1);
    assert_eq!(r.y, 3);

    assert_eq!(
        solve(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
                .to_string()
        )[0],
        12
    );
}
