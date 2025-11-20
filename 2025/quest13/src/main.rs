use std::collections::VecDeque;
use std::fs;

fn main() {
    for (part, rounds) in [(1, 2025), (2, 20252025), (3, 202520252025_usize)] {
        let input = fs::read_to_string(format!("everybody_codes_e2025_q13_p{part}.txt"))
            .expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();

        let mut dial = VecDeque::new();
        let mut dial_len = 1;
        let mut start = 0;
        dial.push_back(1..=1);

        let mut right = true;
        for l in lines {
            let (s, e) = l.split_once("-").unwrap_or((l, l));
            let s = s.parse::<usize>().unwrap();
            let e = e.parse::<usize>().unwrap();
            if right {
                dial.push_back(s..=e);
                right = false;
            } else {
                dial.push_front(s..=e);
                start += 1;
                right = true;
            }
            dial_len += e - s + 1;
        }

        let mut remainder = rounds % dial_len;
        let mut i = start;
        while remainder > 0 {
            let range_len = dial[i].end() - dial[i].start() + 1;
            if range_len <= remainder {
                remainder -= range_len;
                i = (i + 1) % dial.len();
            } else {
                break;
            }
        }
        if i < start {
            println!("{}", dial[i].end() - remainder);
        } else {
            println!("{}", dial[i].start() + remainder);
        }
    }
}
