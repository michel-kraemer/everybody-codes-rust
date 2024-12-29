use std::collections::{HashMap, HashSet};
use std::fs;

fn parse(filename: &str) -> Vec<Vec<u64>> {
    let input = fs::read_to_string(filename).expect("Could not read file");
    let lines = input
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut cols: Vec<Vec<u64>> = Vec::new();
    for l in lines {
        for (x, n) in l.iter().enumerate() {
            if cols.len() < x + 1 {
                cols.resize(x + 1, Vec::new());
            }
            cols[x].push(*n);
        }
    }

    cols
}

fn dance(cols: &mut [Vec<u64>], round: usize) {
    let n_cols = cols.len();
    let clapper = cols[round % n_cols].remove(0);

    let mut i = 0i64;
    let mut di = 1i64;
    for _ in 0..clapper - 1 {
        i += di;
        if i == 0 || i == cols[(round + 1) % n_cols].len() as i64 {
            di = -di;
        }
    }
    cols[(round + 1) % n_cols].insert(i as usize, clapper);
}

fn call(cols: &[Vec<u64>]) -> u64 {
    let mut total = String::new();
    for col in cols {
        total.push_str(&format!("{}", col[0]));
    }
    total.parse::<u64>().unwrap()
}

fn main() {
    // part 1
    let mut cols = parse("everybody_codes_e2024_q05_p1.txt");
    for round in 0..10 {
        dance(&mut cols, round);
    }
    println!("{}", call(&cols));

    // part 2
    let mut cols = parse("everybody_codes_e2024_q05_p2.txt");
    let mut repeats: HashMap<u64, u64> = HashMap::new();
    let mut round = 0;
    let total2 = loop {
        dance(&mut cols, round);
        let c = call(&cols);
        let e = repeats.entry(c).or_default();
        *e += 1;
        round += 1;
        if *e == 2024 {
            break c * round as u64;
        }
    };
    println!("{}", total2);

    // part 3
    let mut cols = parse("everybody_codes_e2024_q05_p3.txt");
    let mut seen = HashSet::new();
    let mut highest = 0u64;
    let mut round = 0;
    loop {
        dance(&mut cols, round);
        let c = call(&cols);
        round += 1;
        if c > highest {
            highest = c;
        }
        if seen.contains(&cols) {
            break;
        }
        seen.insert(cols.clone());
    }
    println!("{}", highest);
}
