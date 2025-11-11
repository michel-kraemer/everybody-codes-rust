use std::fs;

fn make_index(input: &str) -> Vec<Vec<usize>> {
    let mut result = vec![Vec::new(); 128];
    for (i, b) in input.bytes().enumerate() {
        result[b as usize].push(i);
    }
    result
}

fn main() {
    // part 1
    let input =
        fs::read_to_string("everybody_codes_e2025_q06_p1.txt").expect("Could not read file");
    let index = make_index(&input);
    let mut total = 0;
    for &novice in &index['a' as usize] {
        let mentors = &index['A' as usize];
        total += mentors.partition_point(|&j| j < novice);
    }
    println!("{total}");

    // part 2
    let input =
        fs::read_to_string("everybody_codes_e2025_q06_p2.txt").expect("Could not read file");
    let index = make_index(&input);
    let mut total = 0;
    for profession in ['a', 'b', 'c'] {
        for &novice in &index[profession as usize] {
            let mentors = &index[profession.to_ascii_uppercase() as usize];
            total += mentors.partition_point(|&j| j < novice);
        }
    }
    println!("{total}");

    // part 3
    let input =
        fs::read_to_string("everybody_codes_e2025_q06_p3.txt").expect("Could not read file");
    assert!(input.len() > 1000);
    let index = make_index(&input);
    let mut total = 0;
    for profession in ['a', 'b', 'c'] {
        for &novice in &index[profession as usize] {
            let mentors = &index[profession.to_ascii_uppercase() as usize];
            let all = mentors.len();
            if novice < 1000 {
                let min = mentors.partition_point(|&j| j < novice.saturating_sub(1000));
                let min_wrapping = mentors.partition_point(|&j| j < novice + input.len() - 1000);
                let max = mentors.partition_point(|&j| j <= novice + 1000);
                total += max - min;
                total += (all - (min_wrapping - max)) * 999;
            } else if novice >= input.len() - 1000 {
                let min = mentors.partition_point(|&j| j < novice.saturating_sub(1000));
                let max = mentors.partition_point(|&j| j <= novice + 1000);
                let max_wrapping = mentors.partition_point(|&j| j <= novice - input.len() + 1000);
                total += max - min;
                total += (all - (min - max_wrapping)) * 999;
            } else {
                let min = mentors.partition_point(|&j| j < novice - 1000);
                let max = mentors.partition_point(|&j| j <= novice + 1000);
                total += (max - min) * 1000;
            }
        }
    }
    println!("{total}");
}
