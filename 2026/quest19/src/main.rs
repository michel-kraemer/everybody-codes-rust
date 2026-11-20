use itertools::Itertools;
use std::cmp::Reverse;
use std::fs;

fn combine(block1: &[u8], block2: impl Iterator<Item = u8>) -> u64 {
    let mut counts = [0; 26];
    for (&(mut c1), mut c2) in block1.iter().zip(block2) {
        if c2 < c1 {
            (c1, c2) = (c2, c1);
        }

        let r = match (c1, c2) {
            (b'K', _) | (_, b'K') => b'K',
            (b'C', b'M') => b'B',
            (b'C', b'Y') => b'G',
            (b'M', b'Y') => b'R',
            _ => c1,
        };

        counts[(r - b'A') as usize] += 1;
    }

    counts.sort_unstable_by_key(|&c| Reverse(c));
    counts[0] * counts[1] * counts[2]
}

fn main() {
    // part 1
    let input =
        fs::read_to_string("everybody_codes_e2026_q19_p1.txt").expect("Could not read file");

    let (block1, block2) = input.trim().split_once("\n\n").unwrap();
    let block1 = block1.bytes().filter(|b| *b != b'\n').collect::<Vec<_>>();
    let block2 = block2.bytes().filter(|b| *b != b'\n').collect::<Vec<_>>();

    println!("{}", combine(&block1, block2.into_iter()));

    // part 2
    let input =
        fs::read_to_string("everybody_codes_e2026_q19_p2.txt").expect("Could not read file");

    let (block1, block2) = input.trim().split_once("\n\n").unwrap();
    let block1 = block1.bytes().filter(|b| *b != b'\n').collect::<Vec<_>>();
    let block2 = block2
        .bytes()
        .filter(|b| *b != b'\n')
        .map(|b| (b - b'1') as usize)
        .collect::<Vec<_>>();

    let mut max = 0;
    for p in b"CKMY".iter().permutations(4) {
        max = max.max(combine(&block1, block2.iter().map(|&i| *p[i])));
    }
    println!("{max}");

    // part 3
    let input =
        fs::read_to_string("everybody_codes_e2026_q19_p3.txt").expect("Could not read file");
    let block1 = input.bytes().filter(|b| *b != b'\n').collect::<Vec<_>>();

    // count how often 'C', 'M', 'Y', and 'K' appear in the input
    let mut counts = [0_u64; 26];
    for c in block1 {
        counts[(c - b'A') as usize] += 1;
    }
    let mut counts = [
        counts[(b'C' - b'A') as usize],
        counts[(b'M' - b'A') as usize],
        counts[(b'Y' - b'A') as usize],
        counts[(b'K' - b'A') as usize],
    ];

    let sum = counts[0] + counts[1] + counts[2];
    let avg = sum / 3;
    if counts[3] >= avg {
        // 'K' is greater than or equal to the average of the other numbers. We
        // have to use it in our product and distribute the other colors evenly
        // to two numbers.
        let avg = sum / 2;
        if sum % 2 == 0 {
            counts[0] = 0;
            counts[1] = avg;
            counts[2] = avg;
        } else {
            counts[0] = 0;
            counts[1] = avg;
            counts[2] = avg + 1;
        }
    } else {
        // Distribute 'C', 'M', 'Y' evenly. 'K' must not be touched because it
        // cannot be converted to another color.
        if sum % 3 == 0 {
            counts[0] = avg;
            counts[1] = avg;
            counts[2] = avg;
        } else if sum % 3 == 1 {
            counts[0] = avg;
            counts[1] = avg;
            counts[2] = avg + 1;
        } else {
            counts[0] = avg;
            counts[1] = avg + 1;
            counts[2] = avg + 1;
        }
    }

    // multiply the largest three numbers
    counts.sort_unstable();
    println!("{}", counts[1] * counts[2] * counts[3]);
}
