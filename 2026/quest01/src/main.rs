use std::fs;

fn parse(path: &str) -> Vec<u64> {
    let input = fs::read_to_string(path).expect("Could not read file");
    input
        .trim()
        .split(',')
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    // part 1
    let outposts = parse("everybody_codes_e2026_q01_p1.txt");
    let mut total = 0;
    for available_pine_cones in outposts {
        let mut side_length = available_pine_cones.isqrt();
        if side_length * side_length != available_pine_cones {
            side_length += 1;
        }
        let more = side_length * side_length - available_pine_cones;
        total += more;
    }
    println!("{total}");

    // part 2
    let outposts = parse("everybody_codes_e2026_q01_p2.txt");
    let mut total = 0;
    for available_pine_cones in outposts {
        let side_length = (available_pine_cones.div_ceil(4) + 1).max(3);
        let perimeter = side_length * 4 - 4;
        let more = perimeter - available_pine_cones;
        total += more;
    }
    println!("{total}");

    // part 3
    let outposts = parse("everybody_codes_e2026_q01_p3.txt");
    let mut total = 0;
    for available_pine_cones in outposts {
        let mut side_length = available_pine_cones.isqrt() + 1;
        if side_length % 2 == 0 {
            side_length += 1;
        }
        let more = (side_length * side_length - 1) - available_pine_cones;
        total += more;
    }
    println!("{total}");
}
