use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Sword {
    quality: u64,
    segments: Vec<u64>,
    id: u64,
}

fn parse_sword(line: &str) -> Sword {
    struct Segment {
        center: u64,
        left: Option<u64>,
        right: Option<u64>,
    }

    let (id, numbers) = line.split_once(':').unwrap();
    let id = id.parse().unwrap();
    let numbers = numbers.split(',').map(|n| n.parse().unwrap());

    let mut raw_segments: Vec<Segment> = Vec::new();
    for n in numbers {
        let mut found = false;
        for rs in &mut raw_segments {
            if n < rs.center && rs.left.is_none() {
                rs.left = Some(n);
                found = true;
                break;
            } else if n > rs.center && rs.right.is_none() {
                rs.right = Some(n);
                found = true;
                break;
            }
        }
        if !found {
            let node = Segment {
                center: n,
                left: None,
                right: None,
            };
            raw_segments.push(node);
        }
    }

    let mut quality = String::new();
    for n in &raw_segments {
        quality.push_str(&n.center.to_string());
    }
    let quality = quality.parse().unwrap();

    let mut segments = Vec::new();
    for s in &raw_segments {
        let mut value = String::new();
        if let Some(left) = s.left {
            value.push_str(&left.to_string());
        }
        value.push_str(&s.center.to_string());
        if let Some(right) = s.right {
            value.push_str(&right.to_string());
        }
        segments.push(value.parse().unwrap());
    }

    Sword {
        quality,
        segments,
        id,
    }
}

fn main() {
    // part 1
    let input =
        fs::read_to_string("everybody_codes_e2025_q05_p1.txt").expect("Could not read file");
    let sword = parse_sword(input.trim());
    println!("{}", sword.quality);

    // part 2
    let input =
        fs::read_to_string("everybody_codes_e2025_q05_p2.txt").expect("Could not read file");
    let mut swords = input.lines().map(parse_sword).collect::<Vec<_>>();
    swords.sort();
    println!("{}", swords[swords.len() - 1].quality - swords[0].quality);

    // part 3
    let input =
        fs::read_to_string("everybody_codes_e2025_q05_p3.txt").expect("Could not read file");
    let mut swords = input.lines().map(parse_sword).collect::<Vec<_>>();
    swords.sort();
    swords.reverse();

    let mut checksum = 0;
    for (i, s) in swords.into_iter().enumerate() {
        checksum += (i as u64 + 1) * s.id;
    }
    println!("{checksum}");
}
