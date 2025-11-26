use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
};

// Right, Down, Left, Up
const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

struct Input {
    width: usize,
    height: usize,
    grid: Vec<u64>,
    origin: (usize, usize),
    start: (usize, usize),
}

/// Parse the input file and return a tuple with the grid's width, height, and
/// the grid itself
fn parse(path: &str) -> Input {
    let input = fs::read_to_string(path).expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .into_iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let mut origin = (0, 0);
    let mut start = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'@' {
                origin = (x, y);
            } else if grid[y * width + x] == b'S' {
                start = (x, y);
            }
        }
    }

    let grid = grid
        .into_iter()
        .map(|b| {
            if b.is_ascii_digit() {
                (b - b'0') as u64
            } else {
                0
            }
        })
        .collect();

    Input {
        width,
        height,
        grid,
        origin,
        start,
    }
}

fn get_radius(x: usize, y: usize, origin_x: usize, origin_y: usize) -> u64 {
    let dx = origin_x.abs_diff(x) as f64;
    let dy = origin_y.abs_diff(y) as f64;
    (dx * dx + dy * dy).sqrt().ceil() as u64
}

fn main() {
    // part 1
    let Input {
        width,
        height,
        grid,
        origin,
        ..
    } = parse("everybody_codes_e2025_q17_p1.txt");
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            if get_radius(x, y, origin.0, origin.1) <= 10 {
                total += grid[y * width + x];
            }
        }
    }
    println!("{total}");

    // part 2
    let Input {
        width,
        height,
        grid,
        origin,
        ..
    } = parse("everybody_codes_e2025_q17_p2.txt");
    let mut destruction = vec![0; origin.0];
    for y in 0..height {
        for x in 0..width {
            let r = (get_radius(x, y, origin.0, origin.1) - 1) as usize;
            if r < destruction.len() {
                destruction[r] += grid[y * width + x];
            }
        }
    }
    println!(
        "{}",
        destruction
            .into_iter()
            .enumerate()
            .max_by_key(|(_, d)| *d)
            .map(|(r, d)| (r as u64 + 1) * d)
            .unwrap()
    );

    // part 3 ...
    let Input {
        width,
        height,
        grid,
        origin,
        start,
    } = parse("everybody_codes_e2025_q17_p3.txt");

    // Perform Dijkstra's to find two shortest paths (one on the left and one on
    // the right side of the volcano) to each cell at column `origin.0` and
    // below the volcano (i.e. y > `origin.y`). The answer is the first cell
    // that has two shortest paths and where the sum of the costs of these paths
    // is smaller than the time it takes the volcano to get to this cell.
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((
        0,
        start.0,
        start.1,
        get_radius(start.0, start.1, origin.0, origin.1) * 30,
        true,
    )));
    queue.push(Reverse((
        0,
        start.0,
        start.1,
        get_radius(start.0, start.1, origin.0, origin.1) * 30,
        false,
    )));
    let mut seen: HashMap<(usize, usize, u64, bool), u64> = HashMap::default();

    while let Some(Reverse((time, x, y, max_time, is_left))) = queue.pop() {
        if x == origin.0 && y > origin.1 {
            let cost = grid[y * width + origin.0];
            if let Some(other_time) = seen.get(&(x, y, max_time, !is_left)) {
                // we found two shortest paths, check if the sum of the times
                // (minus the costs for the current cell, because it has been
                // visited twice) is lower than the maximum time
                let time_spent = time + other_time - cost;
                if time_spent < max_time {
                    println!("{}", time_spent * (max_time / 30 - 1));
                    break;
                }
            }
        }

        for (dx, dy) in DIRS {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if nx >= 0 && (nx as usize) < width && ny >= 0 && (ny as usize) < height {
                let nx = nx as usize;
                let ny = ny as usize;

                if ny == origin.1 && ((is_left && nx > origin.0) || (!is_left && nx < origin.0)) {
                    // never pass the volcano to the right, if `is_left` is
                    // `true` and vice-versa
                    continue;
                }

                let new_max_time = max_time.min(get_radius(nx, ny, origin.0, origin.1) * 30);

                if (is_left && nx >= origin.0 + new_max_time as usize / 180)
                    || (!is_left && nx <= origin.0 - new_max_time as usize / 180)
                {
                    // optimization: don't go to far to the right if `is_left`
                    // is `true` and vice-versa
                    continue;
                }

                let cost = grid[ny * width + nx];
                let old_time = seen
                    .get(&(nx, ny, new_max_time, is_left))
                    .unwrap_or(&u64::MAX);
                if time + cost < new_max_time && time + cost < *old_time {
                    seen.insert((nx, ny, new_max_time, is_left), time + cost);
                    queue.push(Reverse((time + cost, nx, ny, new_max_time, is_left)));
                }
            }
        }
    }
}
