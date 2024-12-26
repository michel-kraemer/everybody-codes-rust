use std::fs;

fn creature_to_potion(c: u8) -> u64 {
    match c {
        b'B' => 1,
        b'C' => 3,
        b'D' => 5,
        _ => 0,
    }
}

fn main() {
    // part 1
    let input =
        fs::read_to_string("everybody_codes_e2024_q01_p1.txt").expect("Could not read file");
    let bytes = input.trim().as_bytes();

    let mut total = 0;
    let mut i = 0;
    while i < bytes.len() {
        total += creature_to_potion(bytes[i]);
        i += 1;
    }
    println!("{}", total);

    // part 2
    let input =
        fs::read_to_string("everybody_codes_e2024_q01_p2.txt").expect("Could not read file");
    let bytes = input.trim().as_bytes();

    let mut total = 0;
    let mut i = 0;
    while i < bytes.len() - 1 {
        let c1 = bytes[i];
        let c2 = bytes[i + 1];
        total += creature_to_potion(c1);
        total += creature_to_potion(c2);
        if ![c1, c2].contains(&b'x') {
            total += 2;
        }
        i += 2;
    }
    println!("{}", total);

    // part 3
    let input =
        fs::read_to_string("everybody_codes_e2024_q01_p3.txt").expect("Could not read file");
    let bytes = input.trim().as_bytes();

    let mut total = 0;
    let mut i = 0;
    while i < bytes.len() - 2 {
        let c1 = bytes[i];
        let c2 = bytes[i + 1];
        let c3 = bytes[i + 2];
        total += creature_to_potion(c1);
        total += creature_to_potion(c2);
        total += creature_to_potion(c3);
        let xs = [c1, c2, c3].into_iter().filter(|c| *c == b'x').count();
        if xs == 0 {
            total += 6;
        } else if xs == 1 {
            total += 2;
        }
        i += 3;
    }
    println!("{}", total);
}
