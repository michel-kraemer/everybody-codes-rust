use std::{cmp::Reverse, fs};

fn parse(file: &str) -> Vec<(u64, Vec<i32>)> {
    let input = fs::read_to_string(file).expect("Could not read file");
    let mut scales = Vec::new();
    for l in input.lines() {
        let (id, colors) = l.split_once(':').unwrap();
        let id = id.parse::<u64>().unwrap();
        let colors = colors
            .split_ascii_whitespace()
            .map(|c| {
                let mut result = 0;
                for b in c.bytes() {
                    result <<= 1;
                    if b.is_ascii_uppercase() {
                        result |= 1;
                    }
                }
                result
            })
            .collect::<Vec<_>>();
        scales.push((id, colors));
    }
    scales
}

fn main() {
    // part 1
    let scales = parse("everybody_codes_e3_q01_p1.txt");
    println!(
        "{}",
        scales
            .into_iter()
            .filter(|s| s.1[1] > s.1[0] && s.1[1] > s.1[2])
            .map(|s| s.0)
            .sum::<u64>()
    );

    // part 2
    let scales = parse("everybody_codes_e3_q01_p2.txt");
    println!(
        "{}",
        scales
            .into_iter()
            .max_by_key(|s| (s.1[3], Reverse(s.1[0] + s.1[1] + s.1[2])))
            .unwrap()
            .0
    );

    // part 3
    let scales = parse("everybody_codes_e3_q01_p3.txt");
    let mut groups = vec![(0, 0); 6];
    for s in scales {
        let shiny = if s.1[3] <= 30 {
            0
        } else if s.1[3] >= 33 {
            3
        } else {
            continue;
        };

        let dominant_color = if s.1[0] > s.1[1] && s.1[0] > s.1[2] {
            0
        } else if s.1[1] > s.1[0] && s.1[1] > s.1[2] {
            1
        } else if s.1[2] > s.1[0] && s.1[2] > s.1[1] {
            2
        } else {
            continue;
        };

        let g = &mut groups[shiny + dominant_color];
        g.0 += 1;
        g.1 += s.0;
    }
    println!("{}", groups.into_iter().max_by_key(|g| g.0).unwrap().1);
}
