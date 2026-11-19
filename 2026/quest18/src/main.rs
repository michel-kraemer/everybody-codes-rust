use std::fs;

fn parse(path: &str) -> Vec<(usize, usize, usize)> {
    let input = fs::read_to_string(path).expect("Could not read file");
    let mut stacks = Vec::new();
    for l in input.lines() {
        let mut parts = l.split_ascii_whitespace();
        let x = parts.next().unwrap()[2..].parse::<usize>().unwrap();
        let y = parts.next().unwrap()[2..].parse::<usize>().unwrap();
        let w = if let Some(w) = parts.next() {
            w[6..].parse::<usize>().unwrap()
        } else {
            1
        };
        stacks.push((x, y, w));
    }
    stacks.sort_unstable();
    stacks
}

fn compute(stacks: Vec<(usize, usize, usize)>) -> usize {
    let mut fill = 0;
    for (i, s) in stacks.iter().enumerate().skip(1) {
        // Find all 'visible' stacks (skyscrapers) to the left (in increasing
        // order of height). Visible stacks are those whose view is not blocked
        // by other stacks. For each visible stack X, compute how much empty
        // cells between the current stack I and X are visible.
        let mut prev_height = 0;
        for x in (0..i).rev() {
            if stacks[x].1 > prev_height {
                // Stack is visible. Compute empty cells between I and X but
                // only those that are above the previous height.
                let w = s.0 - (stacks[x].0 + stacks[x].2);
                let h = stacks[x].1.min(s.1) - prev_height;
                fill += w * h;
                prev_height = stacks[x].1;
            }

            if stacks[x].1 > s.1 {
                // Stop if X is taller than the current stack. There can be no
                // other visible stack.
                break;
            }
        }
    }

    // count the stacks too
    fill + stacks.iter().map(|s| s.1 * s.2).sum::<usize>()
}

fn main() {
    // part 1
    let stacks = parse("everybody_codes_e2026_q18_p1.txt");
    println!("{}", compute(stacks));

    // part 2
    let stacks = parse("everybody_codes_e2026_q18_p2.txt");
    println!("{}", compute(stacks));

    // part 3
    let stacks = parse("everybody_codes_e2026_q18_p3.txt");
    println!("{}", compute(stacks));
}
