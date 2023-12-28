
#[derive(Debug, PartialEq, Clone)]
enum Mode {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug)]
struct Mapping {
    source: u64,
    dest: u64,
    count: u64,
}

#[derive(Debug)]
struct Seed {
    start: u64,
    count: u64,
}

fn rek(seeds: &mut Vec<Seed>, s: usize, mapping: &[Mapping]) {
    let seed = &seeds[s];
    let seed_end = seed.start + seed.count;

    for m in mapping {
        let m_end = m.source + m.count;
        if seed.start >= m.source && seed.start < m_end {
            // seed-start is in our range
            if seed_end <= m_end {
                // entire seed is in current mapping (see part one)
                let off = seed.start - m.source;
                seeds[s] = Seed {
                    start: m.dest + off,
                    count: seed.count,
                };
                return;
            } else {
                // seed count is too large, split seed up
                let off = seed.start - m.source;
                seeds[s] = Seed {
                    start: m.dest + off,
                    count: m_end - seed.start,
                };
                seeds.push(Seed {
                    start: m_end,
                    count: seed_end - m_end,
                });
                rek(seeds, seeds.len() - 1, mapping);
                return;
            }
        }
    }
}

pub fn solve(input: String) {
    let lines: Vec<_> = input.lines().collect();

    let seeds: Vec<_> = lines[0][7..].split(' ').filter_map(|w| w.parse::<u64>().ok()).collect();

    // Part One: each value is one seed
    let mut seeds_one = Vec::new();
    for s in &seeds {
        let seed = Seed {
            start: *s,
            count: 1,
        };
        seeds_one.push(seed);
    }
    // Part Two: start - range combination
    let mut seeds_two = Vec::new();
    for s in (0..seeds.len()).step_by(2) {
        let seed = Seed {
            start: seeds[s],
            count: seeds[s+1],
        };
        seeds_two.push(seed);
    }

    let mut mappings = Vec::<Vec<_>>::new();
    for _ in 0..7 {
        mappings.push(Vec::new());
    }

    let mut mode = Mode::Seeds;
    for line in lines[1..].iter() {
        // skip empty lines
        if line.is_empty() {
            continue;
        }

        let last_mode = mode.clone();
        mode = match *line {
            "seed-to-soil map:" => Mode::SeedToSoil,
            "soil-to-fertilizer map:" => Mode::SoilToFertilizer,
            "fertilizer-to-water map:" => Mode::FertilizerToWater,
            "water-to-light map:" => Mode::WaterToLight,
            "light-to-temperature map:" => Mode::LightToTemperature,
            "temperature-to-humidity map:" => Mode::TemperatureToHumidity,
            "humidity-to-location map:" => Mode::HumidityToLocation,
            _ => mode,
        };

        // we changed mode in this line => no data
        if last_mode != mode {
            continue
        }

        let range: Vec<u64> = line.split(' ').filter_map(|w| w.parse().ok()).collect();

        let mapping = Mapping {
            dest: range[0],
            source: range[1],
            count: range[2]
        };
        match mode {
            Mode::SeedToSoil => mappings[0].push(mapping),
            Mode::SoilToFertilizer => mappings[1].push(mapping),
            Mode::FertilizerToWater => mappings[2].push(mapping),
            Mode::WaterToLight => mappings[3].push(mapping),
            Mode::LightToTemperature => mappings[4].push(mapping),
            Mode::TemperatureToHumidity => mappings[5].push(mapping),
            Mode::HumidityToLocation => mappings[6].push(mapping),
            _ => unreachable!(),
        }
    }

    for mapping in &mappings {
        for s in 0..seeds_one.len() {
            rek(&mut seeds_one, s, mapping);
        }
        for s in 0..seeds_two.len() {
            rek(&mut seeds_two, s, mapping);
        }
    }

    aoc::print_solution(&[
        seeds_one.iter().map(|s| s.start).min().unwrap(),
        seeds_two.iter().map(|s| s.start).min().unwrap(),
    ])
}
