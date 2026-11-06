use std::fs;

fn parse_line(line: &str) -> (u64, u64) {
    let mut parts = line.split(&[' ', ',', '=']);
    let growth = parts.nth(4).unwrap().parse::<u64>().unwrap();
    let limit = parts.nth(2).unwrap().parse::<u64>().unwrap();
    (growth, limit)
}

/// Simulation for part 1. The input is so small, it doesn't make much sense to
/// optimize this code.
fn simulate_limit_elements(input: String) -> u64 {
    let mut total = 0;
    for l in input.lines() {
        let (growth, limit) = parse_line(l);

        let mut branches = vec![1];
        let mut seconds = 1;
        let mut elements = 1;

        while elements < limit {
            elements = 0;
            let mut new_branches = Vec::new();
            for mut b in branches {
                b += 1;
                if b == growth {
                    new_branches.push(b / 2); // current branch split in the middle
                    new_branches.push(b / 2); // left branch
                    new_branches.push(1); // new right branch
                    elements += (b / 2) * 2 + 2;
                } else {
                    new_branches.push(b);
                    elements += b;
                }
            }
            branches = new_branches;

            seconds += 1;
        }

        total += seconds;
    }
    total
}

/// Simulation for parts 2 and 3
fn simulate_limit_leaves(input: String) -> u64 {
    let mut total = 0;
    for l in input.lines() {
        let (growth, limit) = parse_line(l);

        // Instead of storing each branch individually, we only store how many
        // branches have the same length and has_leaf flag. `branches` is always
        // sorted, so the longest branch is always at the end.
        let mut branches: Vec<(u64, bool, u64)> = vec![(1, true, 1)];

        let mut leaves = 1;
        let mut seconds = 1;
        while leaves < limit {
            // skip ahead in time as much as possible
            let max = branches.last().unwrap().0;
            let skip = growth - max;
            seconds += skip;

            // simulate growth
            leaves = 0;
            let mut new_branches = Vec::with_capacity(branches.len() * 3);
            for (mut length, has_leaf, count) in branches {
                length += skip;
                if length == growth {
                    // current branch split in the middle (without leaf)
                    new_branches.push((length / 2, false, count));

                    // left branch (keeps leaf)
                    new_branches.push((length / 2, has_leaf, count));
                    if has_leaf {
                        leaves += count;
                    }

                    // new right branch (always with leaf)
                    new_branches.push((1, true, count));
                    leaves += count;
                } else {
                    new_branches.push((length, has_leaf, count));
                    if has_leaf {
                        leaves += count;
                    }
                }
            }

            // sort `new_branches` and merge entries with the same length and
            // has_leaf flag
            new_branches.sort_unstable_by_key(|&(length, has_leaf, _)| (length, has_leaf));
            let mut merged: Vec<(u64, bool, u64)> = Vec::with_capacity(new_branches.len());
            for (length, has_leaf, count) in new_branches {
                if let Some((last_length, last_has_leaf, last_count)) = merged.last_mut()
                    && *last_length == length
                    && *last_has_leaf == has_leaf
                {
                    *last_count += count;
                } else {
                    merged.push((length, has_leaf, count));
                }
            }
            branches = merged;
        }

        total += seconds;
    }

    total
}

fn main() {
    let input =
        fs::read_to_string("everybody_codes_e2026_q09_p1.txt").expect("Could not read file");
    println!("{}", simulate_limit_elements(input));

    let input =
        fs::read_to_string("everybody_codes_e2026_q09_p2.txt").expect("Could not read file");
    println!("{}", simulate_limit_leaves(input));

    let input =
        fs::read_to_string("everybody_codes_e2026_q09_p3.txt").expect("Could not read file");
    println!("{}", simulate_limit_leaves(input));
}
