use std::{collections::VecDeque, fs};

fn process_circle(line: &[u8], repeats: usize) -> usize {
    let mut count = 0;
    let line = line.repeat(repeats);
    let (left, right) = line.split_at(line.len() / 2);
    let mut left = left.iter().copied().collect::<VecDeque<_>>();
    let mut right = right.iter().copied().collect::<VecDeque<_>>();

    for &f in b"RGB".iter().cycle() {
        count += 1;
        if left.pop_front().unwrap() == f {
            if (left.len() + right.len()) % 2 != 0 {
                right.pop_front();
            }
        } else if right.len() > left.len() {
            left.push_back(right.pop_front().unwrap());
        }
        if left.is_empty() {
            break;
        }
    }
    count
}

fn main() {
    // part 1
    let input = fs::read_to_string("everybody_codes_e2_q02_p1.txt").expect("Could not read file");
    let mut line = input.trim().as_bytes().iter().collect::<VecDeque<_>>();
    let mut count = 0;
    for &f in b"RGB".iter().cycle() {
        count += 1;
        while !line.is_empty()
            && let Some(&l) = line.pop_front()
            && l == f
        {
            // go ahead
        }
        if line.is_empty() {
            break;
        }
    }
    println!("{count}");

    // part 2
    let input = fs::read_to_string("everybody_codes_e2_q02_p2.txt").expect("Could not read file");
    println!("{}", process_circle(input.trim().as_bytes(), 100));

    // part 3
    let input = fs::read_to_string("everybody_codes_e2_q02_p3.txt").expect("Could not read file");
    println!("{}", process_circle(input.trim().as_bytes(), 100000));
}
