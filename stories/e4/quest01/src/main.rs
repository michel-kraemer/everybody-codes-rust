use std::collections::HashSet;
use std::fs;

fn parse(file: &str) -> Vec<Vec<u64>> {
    let input = fs::read_to_string(file).expect("Could not read file");
    input
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Under,
    Over,
}

impl Side {
    fn flip(&self) -> Self {
        match self {
            Self::Under => Self::Over,
            Self::Over => Self::Under,
        }
    }
}

struct Arc {
    from: u64,
    to: u64,
    side: Side,
}

impl Arc {
    /// Construct a new arc with sorted `from` and `to` positions
    fn new(a: u64, b: u64, side: Side) -> Self {
        if a > b {
            Self {
                from: b,
                to: a,
                side,
            }
        } else {
            Self {
                from: a,
                to: b,
                side,
            }
        }
    }

    /// Return `true` if the given jump is on the same side as this arc, starts
    /// outside this arc, and ends inside it.
    fn jumps_in(&self, jump_from: u64, jump_to: u64, jump_side: Side) -> bool {
        self.side == jump_side
            && !(self.from..=self.to).contains(&jump_from)
            && (self.from + 1..self.to).contains(&jump_to)
    }

    /// Return `true` if the given jump is on the same side as this arc, starts
    /// inside this arc, and ends outside it.
    fn jumps_out(&self, jump_from: u64, jump_to: u64, jump_side: Side) -> bool {
        self.side == jump_side
            && (self.from + 1..self.to).contains(&jump_from)
            && !(self.from..=self.to).contains(&jump_to)
    }

    /// Return `true` if the given jump would cross this arc
    fn crosses(&self, jump_from: u64, jump_to: u64, jump_side: Side) -> bool {
        self.jumps_in(jump_from, jump_to, jump_side)
            || self.jumps_out(jump_from, jump_to, jump_side)
    }
}

fn main() {
    // part 1
    let sequences = parse("everybody_codes_e4_q01_p1.txt");
    let mut total = 0;
    for seq in sequences {
        let mut cur = 0;
        let mut seen = HashSet::new();
        for len in seq {
            seen.insert(cur);
            if len <= cur && !seen.contains(&(cur - len)) {
                cur -= len;
            } else {
                cur += len;
            }
        }
        total += cur;
    }
    println!("{total}");

    // part 2
    let sequences = parse("everybody_codes_e4_q01_p2.txt");
    let mut total = 0;
    for seq in sequences {
        let mut cur = 0;
        let mut seen = HashSet::new();
        for len in seq {
            seen.insert(cur);
            if len <= cur && !seen.contains(&(cur - len)) {
                cur -= len;
            } else {
                cur += len;
                while seen.contains(&cur) {
                    cur += 1;
                }
            }
        }
        total += cur;
    }
    println!("{total}");

    // part 3
    let sequences = parse("everybody_codes_e4_q01_p3.txt");
    let mut total = 0;
    for seq in sequences {
        let mut side = Side::Under;
        let mut arcs: Vec<Arc> = Vec::new();

        let mut cur = 0;
        let mut seen = HashSet::new();
        'outer: for len in seq {
            seen.insert(cur);

            let mut next = cur;
            if len <= next
                && !seen.contains(&(next - len))
                && !arcs.iter().any(|a| a.crosses(cur, next - len, side))
            {
                next -= len;
            } else {
                next += len;
                if arcs.iter().any(|a| a.jumps_out(cur, next, side)) {
                    // We're inside another arc. Increasing `len` wouldn't help.
                    continue 'outer;
                }

                while seen.contains(&next) || arcs.iter().any(|j| j.jumps_in(cur, next, side)) {
                    next += 1;
                    if arcs.iter().any(|a| a.jumps_out(cur, next, side)) {
                        continue 'outer;
                    }
                }
            }

            arcs.push(Arc::new(cur, next, side));

            side = side.flip();
            cur = next;
        }

        total += cur;
    }

    println!("{total}");
}
