use std::fs;

fn main() {
    // part 1
    let input =
        fs::read_to_string("everybody_codes_e2025_q04_p1.txt").expect("Could not read file");
    let lines = input
        .lines()
        .map(|l| l.parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    let mut turns = 2025.0;
    for l in lines.windows(2) {
        let ratio = l[0] / l[1];
        turns *= ratio;
    }
    println!("{}", turns.floor());

    // part 2
    let input =
        fs::read_to_string("everybody_codes_e2025_q04_p2.txt").expect("Could not read file");
    let lines = input
        .lines()
        .map(|l| l.parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    let mut turns = 1.0;
    for l in lines.windows(2) {
        let ratio = l[0] / l[1];
        turns *= ratio;
    }
    println!("{}", (10000000000000.0 / turns).ceil());

    // part 3
    let input =
        fs::read_to_string("everybody_codes_e2025_q04_p3.txt").expect("Could not read file");
    let lines = input
        .lines()
        .map(|line| {
            let (l, r) = if line.contains("|") {
                line.split_once("|").unwrap()
            } else {
                (line, line)
            };
            (l.parse::<f64>().unwrap(), r.parse::<f64>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut turns = 100.0;
    for l in lines.windows(2) {
        let ratio = l[0].1 / l[1].0;
        turns *= ratio;
    }
    println!("{}", turns.floor());
}
