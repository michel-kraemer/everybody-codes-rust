use std::fs;

fn parse(filename: &str) -> Vec<u64> {
    let input = fs::read_to_string(filename).expect("Could not read file");
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn nail(nails: &[u64], target: u64) -> u64 {
    nails.iter().map(|n| n.abs_diff(target)).sum()
}

fn main() {
    let nails = parse("everybody_codes_e2024_q04_p1.txt");
    println!("{}", nail(&nails, *nails.iter().min().unwrap()));

    let nails = parse("everybody_codes_e2024_q04_p2.txt");
    println!("{}", nail(&nails, *nails.iter().min().unwrap()));

    let mut nails = parse("everybody_codes_e2024_q04_p3.txt");
    nails.sort_unstable();
    println!("{}", nail(&nails, nails[nails.len() / 2]));
}
