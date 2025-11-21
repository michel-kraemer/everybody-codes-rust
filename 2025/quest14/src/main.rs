use std::collections::HashMap;
use std::fs;

/// Parse the input file and return a tuple with the grid's width, height, and
/// the grid itself
fn parse(path: &str) -> (usize, usize, Vec<u8>) {
    let input = fs::read_to_string(path).expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();
    (width, height, grid)
}

fn transform(grid: Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let mut new_grid = grid.clone();
    for y in 0..height {
        for x in 0..width {
            let mut diag_active: usize = 0;
            for (dx, dy) in [(1, 1), (1, -1), (-1, -1), (-1, 1)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0
                    && (nx as usize) < width
                    && ny >= 0
                    && (ny as usize) < height
                    && grid[ny as usize * width + nx as usize] == b'#'
                {
                    diag_active += 1;
                }
            }
            if diag_active.is_multiple_of(2) {
                if grid[y * width + x] == b'#' {
                    new_grid[y * width + x] = b'.';
                } else {
                    new_grid[y * width + x] = b'#';
                }
            }
        }
    }
    new_grid
}

fn count_active(grid: &[u8], width: usize, height: usize) -> usize {
    let mut result = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'#' {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    // part 1
    let (width, height, mut grid) = parse("everybody_codes_e2025_q14_p1.txt");
    let mut total = 0;
    for _ in 0..10 {
        grid = transform(grid, width, height);
        total += count_active(&grid, width, height);
    }
    println!("{total}");

    // part 2
    let (width, height, mut grid) = parse("everybody_codes_e2025_q14_p2.txt");
    let mut total = 0;
    for _ in 0..2025 {
        grid = transform(grid, width, height);
        total += count_active(&grid, width, height);
    }
    println!("{total}");

    // part 3
    let (p_width, p_height, pattern) = parse("everybody_codes_e2025_q14_p3.txt");
    let g_width = 34;
    let g_height = 34;
    let mut grid = vec![b'.'; g_width * g_height];

    let mut seen = HashMap::new();
    let mut total = 0;
    let mut round = 1usize;
    let max_rounds = 1_000_000_000;
    while round <= max_rounds {
        grid = transform(grid, g_width, g_height);

        if let Some(old) = seen.get(&grid) {
            let cycle_len = round - old;
            let cycles = (max_rounds - round) / cycle_len;
            let remaining = max_rounds % cycle_len;
            round = max_rounds - remaining;
            total *= cycles + 1;
            seen.clear();
            continue;
        }
        seen.insert(grid.clone(), round);

        let mut found = true;
        'outer: for py in 0..p_height {
            let gy = py + g_height / 2 - p_height / 2;
            for px in 0..p_width {
                let gx = px + g_width / 2 - p_width / 2;
                if grid[gy * g_width + gx] != pattern[py * p_width + px] {
                    found = false;
                    break 'outer;
                }
            }
        }

        if found {
            total += count_active(&grid, g_width, g_height);
        }
        round += 1;
    }

    println!("{total}");
}
