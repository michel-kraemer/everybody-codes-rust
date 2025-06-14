use num::integer::Integer;
use std::fs;

fn parse(part: usize) -> Vec<(i64, i64)> {
    let input = fs::read_to_string(format!("everybody_codes_e1_q03_p{}.txt", part))
        .expect("Could not read file");
    let mut snails = Vec::new();
    for l in input.lines() {
        let (x, y) = l.split_once(" ").unwrap();
        let (_, x) = x.split_once("=").unwrap();
        let (_, y) = y.split_once("=").unwrap();
        let x = x.parse::<i64>().unwrap();
        let y = y.parse::<i64>().unwrap();
        snails.push((x, y));
    }
    snails
}

/// Chinese remainder theorem
///
/// Solve a system of congruences:
///
/// ```text
/// x ≡ remainders[0] mod moduli[0]
/// x ≡ remainders[1] mod moduli[1]
/// ...
/// x ≡ remainders[k] mod moduli[k]
/// ```
fn chinese(remainders: &[i64], moduli: &[i64]) -> i64 {
    // 1. multiply all moduli
    let n = moduli.iter().copied().reduce(|a, b| a * b).unwrap();

    // 2. divide this product by the moduli
    let m = moduli.iter().map(|v| n / v).collect::<Vec<_>>();

    // 3. apply the extended Euclidean algorithm and multiply y_i by m_i
    // to obtain e_i
    let e = moduli
        .iter()
        .zip(m.iter())
        .map(|(r, m)| i64::extended_gcd(r, m).y * m)
        .collect::<Vec<_>>();

    // 4. calculate the sum of the products of remainders_i and e_i
    let mut result = remainders
        .iter()
        .zip(e.iter())
        .map(|(r, e)| r * e)
        .sum::<i64>();

    // 5. make the result positive and as small as possible
    result = result.rem_euclid(n);

    result
}

fn main() {
    // part 1
    let mut part1_snails = parse(1);
    for _ in 0..100 {
        for s in &mut part1_snails {
            if s.1 == 1 {
                while s.0 > 1 {
                    s.0 -= 1;
                    s.1 += 1;
                }
            } else {
                s.0 += 1;
                s.1 -= 1;
            }
        }
    }

    let mut part1 = 0;
    for s in part1_snails {
        part1 += s.1 * 100 + s.0;
    }
    println!("{}", part1);

    // parts 2 and 3
    for part in [2, 3] {
        let snails = parse(part);

        // cycle lengths
        let mut moduli = Vec::new();

        // steps we expect each snail to have moved at the end (mod its cycle len)
        // in other words: steps each snake needs to take to get to the top row
        // (the row where we expect it to be at the end) within the first cycle
        let mut remainders = Vec::new();

        for s in &snails {
            let mut x0 = s.0;
            let mut x1 = s.0;
            let mut y1 = s.1;
            while x0 > 1 {
                x0 -= 1;
            }
            while y1 > 1 {
                x1 += 1;
                y1 -= 1;
            }
            let m = x1 - x0 + 1;
            moduli.push(m);
            remainders.push(s.1 - 1);
        }

        println!("{}", chinese(&remainders, &moduli));
    }
}
