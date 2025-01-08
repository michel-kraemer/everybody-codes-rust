use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

pub const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(PartialEq, Eq)]
struct State {
    seconds: usize,
    x: usize,
    y: usize,
    level: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.seconds.cmp(&other.seconds)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(filename: &str) -> usize {
    let input = fs::read_to_string(filename).expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .into_iter()
        .flat_map(|l| l.as_bytes().iter().copied())
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'E' {
                start = (x, y);
                break;
            }
        }
    }

    let mut seen = vec![usize::MAX; grid.len()];
    let mut queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    queue.push(Reverse(State {
        seconds: 0,
        x: start.0,
        y: start.1,
        level: 0,
    }));

    while let Some(Reverse(State {
        seconds,
        x,
        y,
        level,
    })) = queue.pop()
    {
        if grid[y * width + x] == b'S' {
            return seconds;
        }

        for (dx, dy) in DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            let nc = grid[ny as usize * width + nx as usize];
            if nc != b'#' {
                let next_level = if nc == b'S' { 0 } else { (nc - b'0') as usize };
                let mut ld = level.abs_diff(next_level);
                if ld > 5 {
                    ld = 10 - ld;
                }
                let nseconds = seconds + 1 + ld;
                if seen[ny as usize * width + nx as usize] > nseconds {
                    seen[ny as usize * width + nx as usize] = nseconds;
                    queue.push(Reverse(State {
                        seconds: nseconds,
                        x: nx as usize,
                        y: ny as usize,
                        level: next_level,
                    }));
                }
            }
        }
    }

    unreachable!();
}

fn main() {
    println!("{}", dijkstra("everybody_codes_e2024_q13_p1.txt"));
    println!("{}", dijkstra("everybody_codes_e2024_q13_p2.txt"));
    println!("{}", dijkstra("everybody_codes_e2024_q13_p3.txt"));
}
