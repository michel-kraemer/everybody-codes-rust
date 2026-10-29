use std::fs;

fn decode(n1: u64, mut n2: u64) -> u64 {
    let mut total = 0;
    let mut m1 = n1;
    let mut exp = 0;

    while n2 > 0 {
        let d1 = m1 % 10;
        let d2 = n2 % 10;

        m1 /= 10;
        n2 /= 10;
        if m1 == 0 {
            // cycle digits of n1
            m1 = n1;
        }

        total += d2 * d1.pow(exp);

        exp += 1;
    }

    total
}

fn main() {
    // part 1
    let input =
        fs::read_to_string("everybody_codes_e2026_q03_p1.txt").expect("Could not read file");
    let mut total = 0;
    for l in input.lines() {
        let (n1, n2) = l[4..].split_once(' ').unwrap();
        let n1 = n1.parse::<u64>().unwrap();
        let n2 = n2.parse::<u64>().unwrap();
        total += decode(n1, n2);
    }
    println!("{total}");

    // part 2
    let input =
        fs::read_to_string("everybody_codes_e2026_q03_p2.txt").expect("Could not read file");
    let mut total = 0;
    for l in input.lines() {
        let (n1, n2) = l[4..].split_once(' ').unwrap();
        let n1 = n1.parse::<u64>().unwrap();
        let n2 = n2.parse::<u64>().unwrap();
        total += decode(n1, n2);
    }
    println!("{total}");

    // part 3
    let input =
        fs::read_to_string("everybody_codes_e2026_q03_p3.txt").expect("Could not read file");
    let mut total = 0;
    for l in input.lines() {
        let (n1, n2) = l[5..].split_once(' ').unwrap();
        let n1 = n1.parse::<u64>().unwrap();
        let n2 = n2.parse::<u64>().unwrap();
        let t = decode(n1, n2);

        // brute force: try to find a smaller n2 that renders the same result
        let mut ok = true;
        for other_n2 in 1..n2 {
            let other_t = decode(n1, other_n2);
            if other_t == t {
                ok = false;
                break;
            }
        }

        if ok {
            total += t;
        }
    }
    println!("{total}");
}
