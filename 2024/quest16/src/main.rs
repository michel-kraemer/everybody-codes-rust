use std::collections::HashMap;
use std::fs;

enum LeftLever {
    Up,
    Down,
    None,
}

fn parse(filename: &str) -> (Vec<Vec<Vec<u8>>>, Vec<usize>) {
    let input = fs::read_to_string(filename).expect("Could not read file");
    let (instructions, str_wheels) = input.split_once("\n\n").unwrap();
    let instructions = instructions
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut wheels = Vec::new();
    for s in str_wheels.lines() {
        let n_wheels = (s.len() + 3) / 4;
        if wheels.len() < n_wheels {
            wheels.resize(n_wheels, Vec::new());
        }
        for n in 0..n_wheels {
            let w0 = &s[(n * 4)..(n * 4 + 3)];
            if !w0.trim().is_empty() {
                wheels[n].push(w0.as_bytes().to_vec());
            }
        }
    }

    (wheels, instructions)
}

fn pull_right(wheels: &[Vec<Vec<u8>>], positions: &mut [usize], instructions: &[usize]) {
    for n in 0..wheels.len() {
        positions[n] = (positions[n] + instructions[n]) % wheels[n].len();
    }
}

fn count_coins(wheels: &[Vec<Vec<u8>>], positions: &[usize]) -> u64 {
    let mut result = 0;
    let mut chars = vec![0; 128];
    for n in 0..wheels.len() {
        chars[wheels[n][positions[n]][0] as usize] += 1;
        chars[wheels[n][positions[n]][2] as usize] += 1;
    }
    for cnt in chars {
        if cnt > 2 {
            result += cnt as u64 - 2;
        }
    }
    result
}

fn dfs(
    moves_left: usize,
    positions: Vec<usize>,
    wheels: &[Vec<Vec<u8>>],
    instructions: &[usize],
    cache: &mut HashMap<(usize, Vec<usize>), (u64, u64)>,
) -> (u64, u64) {
    if moves_left == 0 {
        return (0, 0);
    }

    if let Some(c) = cache.get(&(moves_left, positions.clone())) {
        return *c;
    }

    let mut min = u64::MAX;
    let mut max = u64::MIN;

    for left in [LeftLever::Up, LeftLever::Down, LeftLever::None] {
        let mut new_positions = positions.clone();

        // pull left lever
        for n in 0..wheels.len() {
            let dist = match left {
                LeftLever::Up => 1,
                LeftLever::Down => wheels[n].len() - 1,
                LeftLever::None => 0,
            };
            new_positions[n] = (new_positions[n] + dist) % wheels[n].len();
        }

        pull_right(wheels, &mut new_positions, instructions);
        let coins = count_coins(wheels, &new_positions);

        let (a, b) = dfs(moves_left - 1, new_positions, wheels, instructions, cache);
        min = min.min(coins + a);
        max = max.max(coins + b);
    }

    cache.insert((moves_left, positions), (min, max));

    (min, max)
}

fn main() {
    // part 1
    let (wheels1, instructions1) = parse("everybody_codes_e2024_q16_p1.txt");
    println!(
        "{}",
        wheels1
            .iter()
            .enumerate()
            .map(|(i, w)| w[(instructions1[i] * 100) % w.len()]
                .iter()
                .map(|b| *b as char)
                .collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    );

    // part 2
    let (wheels2, instructions2) = parse("everybody_codes_e2024_q16_p2.txt");

    let mut seen = HashMap::new();
    let mut positions2 = vec![0; wheels2.len()];
    seen.insert(positions2.clone(), (0, 0));

    let mut total2 = 0u64;
    let mut i = 0;
    while i < 202420242024u64 {
        pull_right(&wheels2, &mut positions2, &instructions2);
        total2 += count_coins(&wheels2, &positions2);

        i += 1;

        if let Some((old_i, old_coins)) = seen.get(&positions2) {
            let diff_coins = total2 - old_coins;
            let diff_i = i - old_i;
            let remaining_cycles = (202420242024u64 - i) / diff_i;
            i += remaining_cycles * diff_i;
            total2 += remaining_cycles * diff_coins;
            seen.clear();
        } else {
            seen.insert(positions2.clone(), (i, total2));
        }
    }
    println!("{}", total2);

    // part 3
    let (wheels3, instructions3) = parse("everybody_codes_e2024_q16_p3.txt");
    let mut cache3 = HashMap::new();
    let (min, max) = dfs(
        256,
        vec![0usize; wheels3.len()],
        &wheels3,
        &instructions3,
        &mut cache3,
    );
    println!("{} {}", max, min);
}
