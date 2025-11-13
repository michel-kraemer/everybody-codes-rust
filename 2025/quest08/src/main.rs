use std::fs;

fn parse(file: &str) -> Vec<usize> {
    let input = fs::read_to_string(file).expect("Could not read file");
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn count_intersections(new_thread: (usize, usize), threads: &[(usize, usize)]) -> usize {
    let r = new_thread.0 + 1..new_thread.1;
    let mut result = 0;
    for &old_thread in threads {
        if old_thread == new_thread || {
            (old_thread.0 < new_thread.0 && r.contains(&old_thread.1))
                || (r.contains(&old_thread.0) && old_thread.1 > new_thread.1)
        } {
            result += 1;
        }
    }
    result
}

fn main() {
    // part 1
    let input = parse("everybody_codes_e2025_q08_p1.txt");
    let mut total = 0;
    for w in input.windows(2) {
        if w[0].abs_diff(w[1]) == 16 {
            total += 1;
        }
    }
    println!("{total}");

    // part 2
    let input = parse("everybody_codes_e2025_q08_p2.txt");
    let mut threads: Vec<(usize, usize)> = Vec::new();
    let mut total = 0;
    for w in input.windows(2) {
        let new_thread = if w[0] < w[1] {
            (w[0], w[1])
        } else {
            (w[1], w[0])
        };
        total += count_intersections(new_thread, &threads);
        threads.push(new_thread);
    }
    println!("{total}");

    // part 3
    let input = parse("everybody_codes_e2025_q08_p3.txt");
    let mut threads: Vec<(usize, usize)> = Vec::new();
    let mut total = 0;
    for w in input.windows(2) {
        let new_thread = if w[0] < w[1] {
            (w[0], w[1])
        } else {
            (w[1], w[0])
        };
        threads.push(new_thread);
    }
    for a in 1..=256 {
        for b in a + 1..=256 {
            total = total.max(count_intersections((a, b), &threads));
        }
    }
    println!("{total}");
}
