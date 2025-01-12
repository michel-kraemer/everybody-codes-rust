use std::{collections::VecDeque, fs};

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn parse(filename: &str) -> (Vec<u8>, usize, usize) {
    let input = fs::read_to_string(filename).expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .into_iter()
        .flat_map(|l| l.as_bytes().iter().copied())
        .collect::<Vec<_>>();
    (grid, width, height)
}

fn main() {
    for part in [1, 2] {
        let (grid, width, height) = parse(&format!("everybody_codes_e2024_q18_p{}.txt", part));

        // find entrances
        let mut queue = VecDeque::new();
        let mut seen = vec![false; width * height];
        for y in [0, height - 1] {
            for x in 0..width {
                if grid[y * width + x] == b'.' {
                    queue.push_back((x, y, 0));
                    seen[y * width + x] = true;
                }
            }
        }
        for x in [0, width - 1] {
            for y in 1..height - 1 {
                if grid[y * width + x] == b'.' {
                    queue.push_back((x, y, 0));
                    seen[y * width + x] = true;
                }
            }
        }

        // count plants
        let plants = grid.iter().filter(|&c| *c == b'P').count();

        // flood fill
        let mut collected = 0;
        while let Some((x, y, steps)) = queue.pop_front() {
            if grid[y * width + x] == b'P' {
                collected += 1;
            }
            if collected == plants {
                // the number of steps taken to the last plant is the answer
                println!("{}", steps);
                break;
            }
            for (dx, dy) in DIRS {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0
                    && ny >= 0
                    && nx < width as i32
                    && ny < height as i32
                    && grid[ny as usize * width + nx as usize] != b'#'
                    && !seen[ny as usize * width + nx as usize]
                {
                    seen[ny as usize * width + nx as usize] = true;
                    queue.push_back((nx as usize, ny as usize, steps + 1));
                }
            }
        }
    }

    // part 3
    let (grid, width, height) = parse("everybody_codes_e2024_q18_p3.txt");

    // find plants
    let mut queue = VecDeque::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'P' {
                let plant_id = queue.len();
                queue.push_back((x, y, 0, plant_id));
            }
        }
    }

    // flood fill from each plant and sum up steps taken to each empty cell
    let mut seen = vec![false; queue.len() * width * height];
    let mut step_sums = vec![0; width * height];
    while let Some((x, y, steps, plant_id)) = queue.pop_front() {
        for (dx, dy) in DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                let gi = ny as usize * width + nx as usize;
                let si = plant_id * width * height + gi;
                let c = grid[gi];
                if c != b'#' && !seen[si] {
                    seen[si] = true;
                    if c != b'P' {
                        step_sums[gi] += steps + 1;
                    }
                    queue.push_back((nx as usize, ny as usize, steps + 1, plant_id));
                }
            }
        }
    }

    // find the cell with the lowest sum
    println!(
        "{}",
        step_sums.into_iter().filter(|&s| s > 0).min().unwrap()
    );
}
