use z3::*;
use z3::ast::*;

const DAY: u8 = 24;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Hailstone {
    x: f64,
    y: f64,
    velx: f64,
    vely: f64,
    m: f64,
}

#[allow(dead_code)]
pub fn solve() {
    let input = aoc::input(DAY);

    // Z3
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let sx = Int::new_const(&ctx, "x");
    let sy = Int::new_const(&ctx, "y");
    let sz = Int::new_const(&ctx, "z");
    let svx = Int::new_const(&ctx, "vx");
    let svy = Int::new_const(&ctx, "vy");
    let svz = Int::new_const(&ctx, "vz");

    let mut hailstones = Vec::new();

    // let box_max = 27f64;
    // let box_min = 7f64;
    let box_max = 400000000000000f64;
    let box_min = 200000000000000f64;

    for (i, line) in input.lines().enumerate() {
        let (pos_raw, vel_raw) = line.split_once(" @ ").unwrap();
        let pos: Vec<_> = pos_raw.split(',').map(|p| p.trim().parse::<f64>().unwrap()).collect();
        let vel: Vec<_> = vel_raw.split(',').map(|p| p.trim().parse::<f64>().unwrap()).collect();

        hailstones.push(Hailstone{
            x: pos[0],
            y: pos[1],
            velx: vel[0],
            vely: vel[1],
            // https://stackoverflow.com/questions/73079419/intersection-of-two-vector
            m: vel[1] / vel[0],
        });

        // https://github.com/prove-rs/z3.rs/blob/master/z3/tests/lib.rs
        let hx = Int::from_i64(&ctx, pos[0] as i64);
        let hy = Int::from_i64(&ctx, pos[1] as i64);
        let hz = Int::from_i64(&ctx, pos[2] as i64);
        let hvx = Int::from_i64(&ctx, vel[0] as i64);
        let hvy = Int::from_i64(&ctx, vel[1] as i64);
        let hvz = Int::from_i64(&ctx, vel[2] as i64);
        let zero = Int::from_i64(&ctx, 0);
        let t = Int::new_const(&ctx, format!("t{i}"));
        // time should be positive
        solver.assert(&t.ge(&zero));
        // positions after t
        let hxt = Int::add(&ctx, &[&hx, &Int::mul(&ctx, &[&hvx, &t])]);
        let sxt = Int::add(&ctx, &[&sx, &Int::mul(&ctx, &[&svx, &t])]);
        let hyt = Int::add(&ctx, &[&hy, &Int::mul(&ctx, &[&hvy, &t])]);
        let syt = Int::add(&ctx, &[&sy, &Int::mul(&ctx, &[&svy, &t])]);
        let hzt = Int::add(&ctx, &[&hz, &Int::mul(&ctx, &[&hvz, &t])]);
        let szt = Int::add(&ctx, &[&sz, &Int::mul(&ctx, &[&svz, &t])]);
        // assert that after t, stone hits the hailstone
        solver.assert(&hxt._eq(&sxt));
        solver.assert(&hyt._eq(&syt));
        solver.assert(&hzt._eq(&szt));

    }

    let mut count = 0;

    // get intersections
    // https://stackoverflow.com/questions/73079419/intersection-of-two-vector
    for (i, h1) in hailstones.iter().enumerate() {
        for h2 in hailstones.iter().skip(i+1) {
            let intersection_x = (h1.x * h1.m - h2.x * h2.m + h2.y - h1.y) / (h1.m - h2.m);
            let intersection_y = h1.m * (intersection_x - h1.x) + h1.y;

            // h1 intersection was in the past
            if (intersection_x - h1.x) / h1.velx < 0.0 {
                continue;
            }
            // h2 intersection was in the past
            if (intersection_x - h2.x) / h2.velx < 0.0 {
                continue;
            }

            if box_min <= intersection_x && intersection_x <= box_max && box_min <= intersection_y && intersection_y <= box_max {
                count += 1;
            }
        }
    }

    // https://github.com/prove-rs/z3.rs/blob/master/z3/tests/lib.rs
    assert_eq!(solver.check(), SatResult::Sat);
    let model = solver.get_model().unwrap();
    let sxv = model.eval(&sx, true).unwrap().as_i64().unwrap();
    let syv = model.eval(&sy, true).unwrap().as_i64().unwrap();
    let szv = model.eval(&sz, true).unwrap().as_i64().unwrap();

    aoc::print_solution(DAY, &[count, sxv + syv + szv]);
}
