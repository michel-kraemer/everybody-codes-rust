use std::fs;

/// Read the grid from the given file and convert it to a Vec of u64 values
/// where each value represents a row and each bit represents a cell. '#' will
/// be converted to 1 and '.' to 0.
fn parse(path: &str) -> Vec<u64> {
    let input = fs::read_to_string(path).expect("Could not read file");
    let mut result = Vec::new();
    for l in input.lines() {
        let mut u = 0u64;
        for (i, b) in l.as_bytes().iter().enumerate() {
            if *b == b'#' {
                u |= 1 << i;
            }
        }
        result.push(u);
    }
    result
}

/// Transform the given grid according to the rules from the problem description
/// and create a new grid.
fn transform(grid: Vec<u64>) -> Vec<u64> {
    let mut new_grid = Vec::new();
    for y in 0..grid.len() {
        // select the previous row or 0 if there is no previous row
        let a = if y == 0 { 0 } else { grid[y - 1] };

        // select the current row
        let b = grid[y];

        // select the next row or 0 if there is no next row
        let c = if y == grid.len() - 1 { 0 } else { grid[y + 1] };

        // Use NOT XOR to check which bits in the previous and next row have the
        // same value, i.e. where the number of 1's is even (either no 1's at
        // all or two 1's). This will result in a value where all bit positions
        // with an even number of 1's are set to 1, and all positions with an
        // odd number of 1's to 0.
        let d = !(a ^ c);

        // Shift d to the right and to the left and perform NOT XOR again. This
        // will compare the values to the left and to the right of each bit. If
        // the values are equal (i.e. if there are either no or two 1's), the
        // resulting value will contain a 1 at that position. In other words,
        // even+even=even (1+1=1), odd+odd=even (0+0=1), and even+odd=odd (1+0=0).
        let e = !((d >> 1) ^ (d.rotate_left(1)));

        // XOR with the current value to flip the bits where e contains a 1.
        // Then mask out the unneeded bits.
        let r = (b ^ e) & ((1 << grid.len()) - 1);

        new_grid.push(r);
    }
    new_grid
}

fn count_active(grid: &[u64]) -> u64 {
    grid.iter().map(|v| v.count_ones() as u64).sum::<u64>()
}

fn main() {
    // part 1
    let mut grid = parse("everybody_codes_e2025_q14_p1.txt");
    let mut total = 0;
    for _ in 0..10 {
        grid = transform(grid);
        total += count_active(&grid);
    }
    println!("{total}");

    // part 2
    let mut grid = parse("everybody_codes_e2025_q14_p2.txt");
    let mut total = 0;
    for _ in 0..2025 {
        grid = transform(grid);
        total += count_active(&grid);
    }
    println!("{total}");

    // part 3
    let pattern = parse("everybody_codes_e2025_q14_p3.txt");
    let mut grid = vec![0; 34];
    let offset = grid.len() / 2 - pattern.len() / 2;
    let mask = (1 << pattern.len()) - 1;

    let mut first: Option<Vec<u64>> = None;
    let mut total = 0;
    let mut round = 0_u64;
    let max_rounds = 1_000_000_000;
    while round < max_rounds {
        grid = transform(grid);

        if let Some(first) = &first {
            if grid == *first {
                let cycles = (max_rounds - round) / round;
                let remaining = max_rounds % round;
                round = max_rounds - remaining;
                total *= cycles + 1;
                continue;
            }
        } else {
            first = Some(grid.clone());
        }

        let mut found = true;
        for (y, p) in pattern.iter().enumerate() {
            if (grid[y + offset] >> offset) & mask != *p {
                found = false;
                break;
            }
        }

        if found {
            total += count_active(&grid);
        }
        round += 1;
    }

    println!("{total}");
}
