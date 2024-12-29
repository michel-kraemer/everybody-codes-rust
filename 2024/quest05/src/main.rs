use std::collections::{HashMap, HashSet};
use std::fs;

fn parse(filename: &str) -> Vec<Vec<u64>> {
    let mut cols = vec![Vec::new(); 4];
    let input = fs::read_to_string(filename).expect("Could not read file");
    for l in input.lines() {
        for (i, n) in l.split_whitespace().enumerate() {
            let n = n.parse::<u64>().unwrap();
            cols[i].push(n);
        }
    }
    cols
}

fn dance(cols: &mut [Vec<u64>], round: usize) {
    let clapper = cols[round % 4].remove(0);
    let cl = cols[(round + 1) % 4].len() as u64;
    let i = if ((clapper - 1) / cl) % 2 == 0 {
        (clapper - 1) % cl
    } else {
        cl - (clapper - 1) % cl
    };
    cols[(round + 1) % 4].insert(i as usize, clapper);
}

fn call(cols: &[Vec<u64>]) -> u64 {
    let mut result = 0;
    for col in cols {
        result *= 10u64.pow(col[0].ilog10() + 1);
        result += col[0];
    }
    result
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
