use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

pub const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

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

fn part1() -> u32 {
    #[derive(PartialEq, Eq)]
    struct State {
        alt: u32,
        time: u32,
        x: usize,
        y: usize,
        dir: usize,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            self.alt.cmp(&other.alt).then(other.time.cmp(&self.time))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let (grid, width, height) = parse("everybody_codes_e2024_q20_p1.txt");

    let mut queue = BinaryHeap::new();
    let mut seen = vec![0; width * height];

    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'S' {
                queue.push(State {
                    alt: 1000,
                    time: 0,
                    x,
                    y,
                    dir: 1, // DOWN
                });
                break;
            }
        }
    }

    while let Some(State {
        alt,
        time,
        x,
        y,
        dir: cur_dir,
    }) = queue.pop()
    {
        if time == 100 {
            return alt;
        }

        for (di, &(dx, dy)) in DIRS.iter().enumerate() {
            if (di + 2) % 4 == cur_dir {
                continue;
            }

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && ny >= 0
                && nx < width as i32
                && ny < height as i32
                && grid[ny as usize * width + nx as usize] != b'#'
            {
                let na = match grid[ny as usize * width + nx as usize] {
                    b'S' | b'.' => alt - 1,
                    b'-' => alt - 2,
                    b'+' => alt + 1,
                    _ => panic!(),
                };

                if seen[ny as usize * width + nx as usize] < na {
                    seen[ny as usize * width + nx as usize] = na;
                    queue.push(State {
                        alt: na,
                        time: time + 1,
                        x: nx as usize,
                        y: ny as usize,
                        dir: di,
                    });
                }
            }
        }
    }

    unreachable!()
}

fn part2() -> u32 {
    #[derive(PartialEq, Eq)]
    struct State {
        estimated_remaining_time: u32,
        time: u32,
        n_visited: usize,
        alt: u32,
        x: usize,
        y: usize,
        dir: usize,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .estimated_remaining_time
                .cmp(&self.estimated_remaining_time)
                .then(other.time.cmp(&self.time))
                .then(self.n_visited.cmp(&other.n_visited))
                .then(self.alt.cmp(&other.alt))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let (grid, width, height) = parse("everybody_codes_e2024_q20_p2.txt");

    let mut start = (0, 0);
    let mut a = (0, 0);
    let mut b = (0, 0);
    let mut c = (0, 0);

    for y in 0..height {
        for x in 0..width {
            match grid[y * width + x] {
                b'S' => start = (x, y),
                b'A' => a = (x, y),
                b'B' => b = (x, y),
                b'C' => c = (x, y),
                _ => {}
            }
        }
    }

    let dab = b.0.abs_diff(a.0) + b.1.abs_diff(a.1);
    let dbc = c.0.abs_diff(b.0) + c.1.abs_diff(b.1);
    let dcs = start.0.abs_diff(c.0) + start.1.abs_diff(c.1);

    let mut queue = BinaryHeap::new();
    queue.push(State {
        estimated_remaining_time: 0,
        time: 0,
        n_visited: 0,
        alt: 10000,
        x: start.0,
        y: start.1,
        dir: 1,
    });
    let mut seen_alt = vec![0; width * height];
    let mut seen_visited = vec![0; width * height];

    while let Some(State {
        time,
        n_visited,
        alt,
        x,
        y,
        dir: cur_dir,
        ..
    }) = queue.pop()
    {
        if time > 0 && grid[y * width + x] == b'S' && alt == 10000 {
            return time;
        }

        for (di, &(dx, dy)) in DIRS.iter().enumerate() {
            if (di + 2) % 4 == cur_dir {
                continue;
            }

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && ny >= 0
                && nx < width as i32
                && ny < height as i32
                && grid[ny as usize * width + nx as usize] != b'#'
            {
                let (na, nv) = match grid[ny as usize * width + nx as usize] {
                    b'A' => {
                        if n_visited != 0 {
                            continue;
                        }
                        (alt - 1, n_visited + 1)
                    }
                    b'B' => {
                        if n_visited != 1 {
                            continue;
                        }
                        (alt - 1, n_visited + 1)
                    }
                    b'C' => {
                        if n_visited != 2 {
                            continue;
                        }
                        (alt - 1, n_visited + 1)
                    }
                    b'S' => {
                        if n_visited != 3 {
                            continue;
                        }
                        (alt - 1, n_visited)
                    }
                    b'.' => (alt - 1, n_visited),
                    b'-' => (alt - 2, n_visited),
                    b'+' => (alt + 1, n_visited),
                    _ => panic!(),
                };

                if seen_alt[ny as usize * width + nx as usize] < na
                    || seen_visited[ny as usize * width + nx as usize] < nv
                {
                    let unx = nx as usize;
                    let uny = ny as usize;

                    seen_alt[ny as usize * width + nx as usize] = na;
                    seen_visited[ny as usize * width + nx as usize] = nv;

                    let dist_to_goal = match nv {
                        0 => unx.abs_diff(a.0) + uny.abs_diff(a.1) + dab + dbc + dcs,
                        1 => unx.abs_diff(b.0) + uny.abs_diff(b.1) + dbc + dcs,
                        2 => unx.abs_diff(c.0) + uny.abs_diff(c.1) + dcs,
                        _ => unx.abs_diff(start.0) + uny.abs_diff(start.1),
                    } as u32;
                    let new_estimated_time = time + 1 + dist_to_goal;

                    queue.push(State {
                        estimated_remaining_time: new_estimated_time,
                        time: time + 1,
                        n_visited: nv,
                        alt: na,
                        x: unx,
                        y: uny,
                        dir: di,
                    });
                }
            }
        }
    }

    unreachable!()
}

/// This is most definitely not how you should solve part 3 in general, but it
/// worked for my specific input and is extremely fast. I solved it by looking
/// at my input and finding a reasonable path by hand.
fn part3() -> usize {
    let (grid, width, height) = parse("everybody_codes_e2024_q20_p3.txt");

    let mut start = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'S' {
                start = (x, y);
                break;
            }
        }
    }

    let mut alt = 384400;
    let mut x = start.0;
    let mut y = start.1;
    x += 1;
    alt -= 1;
    x += 1;
    alt -= 1;
    while alt > 0 {
        y += 1;
        if grid[(y % height) * width + x] == b'+' {
            alt += 1;
        } else {
            alt -= 1;
        }
    }
    y
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
    println!("{}", part3());
}
