use std::fs;

fn parse(file: &str) -> Vec<u64> {
    let input = fs::read_to_string(file).unwrap_or_else(|_| panic!("Unable to read file {file}"));
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn main() {
    // part 1
    let mut numbers = parse("everybody_codes_e2025_q03_p1.txt");
    numbers.sort();
    numbers.dedup();
    println!("{}", numbers.into_iter().sum::<u64>());

    // part 2
    let mut numbers = parse("everybody_codes_e2025_q03_p2.txt");
    numbers.sort();
    numbers.dedup();
    println!("{}", numbers[0..20].iter().sum::<u64>());

    // part 3
    let mut numbers = parse("everybody_codes_e2025_q03_p3.txt");
    numbers.sort();

    let mut sets: Vec<u64> = Vec::new();
    for &n in &numbers {
        let i = sets.partition_point(|&m| m >= n);
        if i < sets.len() {
            sets[i] = n;
        } else {
            sets.push(n);
        }
    }

    println!("{}", sets.len());
}
