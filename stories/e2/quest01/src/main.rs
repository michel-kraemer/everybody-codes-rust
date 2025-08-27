use std::{cmp::Reverse, collections::BinaryHeap, fs};

fn read_input(filename: &str) -> (Vec<Vec<bool>>, Vec<Vec<u8>>) {
    let input = fs::read_to_string(filename).expect("Could not read file");
    let (machine, sequences) = input.split_once("\n\n").unwrap();

    let machine = machine
        .lines()
        .map(|l| l.chars().map(|c| c == '*').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let sequences = sequences
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    (machine, sequences)
}

fn drop_token(slot: usize, sequence: &[u8], machine: &[Vec<bool>]) -> usize {
    let mut j = 0;
    let mut x = slot * 2;
    let mut y = 0;
    loop {
        while y < machine.len() && !machine[y][x] {
            y += 1;
        }
        if y == machine.len() {
            break;
        }

        if sequence[j] == b'L' {
            if x == 0 {
                x = 1;
            } else {
                x -= 1;
            }
        } else if x == machine[y].len() - 1 {
            x = machine[y].len() - 2;
        } else {
            x += 1;
        }

        j += 1;
    }

    ((x / 2 + 1) * 2).saturating_sub(slot + 1)
}

fn dijkstra<T, W, U>(scores: &[Vec<usize>], wrap: W, unwrap: U) -> usize
where
    W: Fn(usize) -> T,
    U: Fn(T) -> usize,
    T: Ord,
{
    let mut queue = BinaryHeap::new();
    for (i, &s) in scores[0].iter().enumerate() {
        queue.push((wrap(s), 1, 1 << i));
    }

    while let Some((score, y, seen)) = queue.pop() {
        let score = unwrap(score);
        if y == scores.len() {
            return score;
        }
        for (i, &s) in scores[y].iter().enumerate() {
            if seen & (1 << i) > 0 {
                continue;
            }
            queue.push((wrap(score + s), y + 1, seen | (1 << i)));
        }
    }

    unreachable!()
}

fn main() {
    // part 1
    let (machine, sequences) = read_input("everybody_codes_e2_q01_p1.txt");
    let mut part1 = 0;
    for (slot, sequence) in sequences.iter().enumerate() {
        part1 += drop_token(slot, sequence, &machine);
    }
    println!("{part1}");

    // part 2
    let (machine, sequences) = read_input("everybody_codes_e2_q01_p2.txt");
    let mut part2 = 0;
    for sequence in &sequences {
        let mut max = 0;
        for slot in 0..machine[0].len().div_ceil(2) {
            max = max.max(drop_token(slot, sequence, &machine));
        }
        part2 += max;
    }
    println!("{part2}");

    // part 3
    let (machine, sequences) = read_input("everybody_codes_e2_q01_p3.txt");
    let mut scores: Vec<Vec<usize>> = Vec::new();
    for sequence in &sequences {
        let mut row = Vec::new();
        for slot in 0..machine[0].len().div_ceil(2) {
            row.push(drop_token(slot, sequence, &machine));
        }
        scores.push(row);
    }
    println!(
        "{} {}",
        dijkstra(&scores, Reverse, |s| s.0),
        dijkstra(&scores, |s| s, |s| s)
    );
}
