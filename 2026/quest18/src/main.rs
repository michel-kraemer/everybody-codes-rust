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
        // Find all 'visible' stacks to the left. Visible stacks are those whose
        // view is not blocked by other stacks.
        let mut prev_stacks = Vec::new();
        let mut max = 0;
        for x in (0..i).rev() {
            let m = s.1.min(stacks[x].1);
            if m > max {
                // stack is visible
                prev_stacks.push(stacks[x]);
                max = m;
            }

            if stacks[x].1 > s.1 {
                // Stop if stack is taller than the current stack. There can be
                // no other visible stack.
                break;
            }
        }

        // fill gaps
        let mut prev_height = 0;
        for p in prev_stacks {
            let w = s.0 - (p.0 + p.2);
            let h = p.1.min(max) - prev_height;
            fill += w * h;
            prev_height = p.1;
        }
    }

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
