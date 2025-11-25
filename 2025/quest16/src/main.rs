use std::fs;

fn parse(path: &str) -> Vec<u64> {
    let input = fs::read_to_string(path).expect("Could not read file");
    input
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect()
}

/// Reconstruct the spell from the beginning of a wall
fn reconstruct(mut input: Vec<u64>) -> Vec<u64> {
    let mut result = Vec::new();
    for i in 0..input.len() {
        let n = input[i];
        if n > 0 {
            result.push((i + 1) as u64);
            for j in (i..input.len()).step_by(i + 1) {
                input[j] -= 1;
            }
        }
    }
    result
}

fn main() {
    // part 1
    let input = parse("everybody_codes_e2025_q16_p1.txt");
    let mut total = 0;
    for i in input {
        total += 90 / i;
    }
    println!("{total}");

    // part 2
    let input = parse("everybody_codes_e2025_q16_p2.txt");
    let original = reconstruct(input);
    println!("{}", original.iter().product::<u64>());

    // part 3
    let input = parse("everybody_codes_e2025_q16_p3.txt");
    let original = reconstruct(input);

    // perform binary search to find the maximum length of the wall that would
    // require at most `n_blocks` blocks
    let n_blocks = 202520252025000;
    let mut low = 1;
    let mut high = 1_000_000_000_000_000_000;
    while low + 1 < high {
        let mid = (low + high) / 2;
        let mut required_blocks = 0;
        for &o in &original {
            required_blocks += mid / o;
        }
        if required_blocks > n_blocks {
            high = mid;
        } else {
            low = mid;
        }
    }
    println!("{low}");
}
