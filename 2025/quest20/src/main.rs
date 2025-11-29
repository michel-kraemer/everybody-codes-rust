use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

/// A point in 3D space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D {
    /// The grid's id
    gi: usize,

    // The x coordinate
    x: usize,

    // The y coordinate
    y: usize,
}

impl Point3D {
    fn new(gi: usize, x: usize, y: usize) -> Self {
        Self { gi, x, y }
    }
}

#[derive(PartialEq, Eq)]
struct State {
    steps: u64,
    pos: Point3D,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

/// Part 1: Count unique number of trampoline pairs
fn count_pairs(grid: &[u8], width: usize, height: usize) -> usize {
    let mut unique = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] != b'T' {
                continue;
            }

            let dirs = if y % 2 == x % 2 {
                // up, left, right
                [(0, -1), (-1, 0), (1, 0)]
            } else {
                // down, left, right
                [(0, 1), (-1, 0), (1, 0)]
            };

            for (dx, dy) in dirs {
                let nx = x as i64 + dx;
                let ny = y as i64 + dy;
                if nx >= 0
                    && (nx as usize) < width
                    && ny >= 0
                    && (ny as usize) < height
                    && grid[ny as usize * width + nx as usize] == b'T'
                {
                    let mut from = (x, y);
                    let mut to = (nx as usize, ny as usize);
                    if to < from {
                        std::mem::swap(&mut from, &mut to);
                    }
                    unique.insert((from, to));
                }
            }
        }
    }
    unique.len()
}

/// Rotate a grid by 120°
fn rotate(grid: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut new_grid = grid.to_vec();
    let mut source = (height - 1, height - 1);

    for y in 0..height {
        let mut s = source;
        for (step, x) in (y..width - y).enumerate() {
            new_grid[y * width + x] = grid[s.1 * width + s.0];
            if step % 2 == 0 {
                s.1 = s.1.saturating_sub(1);
            } else {
                s.0 = s.0.saturating_sub(1);
            }
        }
        source.0 += 1;
        source.1 = source.1.saturating_sub(1);
    }

    new_grid
}

/// Create a map containing the jumps from a certain position in 3D to all its
/// possible neighbors. If `can_stay` is `true`, also try if we can stay at the
/// same place while the grid rotates under us.
fn make_map(
    grids: &[Vec<u8>],
    width: usize,
    height: usize,
    can_stay: bool,
) -> HashMap<Point3D, Vec<Point3D>> {
    let up = if can_stay {
        // stay, up, left, right
        vec![(0, 0), (0, -1), (-1, 0), (1, 0)]
    } else {
        // up, left, right
        vec![(0, -1), (-1, 0), (1, 0)]
    };

    let down = if can_stay {
        // stay, down, left, right
        vec![(0, 0), (0, 1), (-1, 0), (1, 0)]
    } else {
        // down, left, right
        vec![(0, 1), (-1, 0), (1, 0)]
    };

    let mut map: HashMap<Point3D, Vec<Point3D>> = HashMap::new();
    for gi in 0..grids.len() {
        let ngi = (gi + 1) % grids.len();
        for y in 0..height {
            for x in 0..width {
                let c = grids[gi][y * width + x];
                if c == b'#' || c == b'.' {
                    continue;
                }

                let dirs = if y % 2 == x % 2 { &up } else { &down };
                for (dx, dy) in dirs {
                    let nx = x as i64 + dx;
                    let ny = y as i64 + dy;
                    if nx >= 0
                        && (nx as usize) < width
                        && ny >= 0
                        && (ny as usize) < height
                        && grids[ngi][ny as usize * width + nx as usize] != b'#'
                        && grids[ngi][ny as usize * width + nx as usize] != b'.'
                    {
                        map.entry(Point3D::new(gi, x, y))
                            .or_default()
                            .push(Point3D::new(ngi, nx as usize, ny as usize));
                    }
                }
            }
        }
    }

    map
}

/// Find the shortest path from the start position in the first grid to one of
/// the end positions in any grid. Return the number of steps needed.
fn shortest_path(
    grids: &[Vec<u8>],
    width: usize,
    height: usize,
    map: &HashMap<Point3D, Vec<Point3D>>,
) -> u64 {
    // find start in first grid
    let mut start = Point3D::new(0, 0, 0);
    for y in 0..height {
        for x in 0..width {
            if grids[0][y * width + x] == b'S' {
                start.x = x;
                start.y = y;
                break;
            }
        }
    }

    // find end positions in all grids
    let mut ends = Vec::new();
    for (gi, grid) in grids.iter().enumerate() {
        let mut end = Point3D::new(gi, 0, 0);
        for y in 0..height {
            for x in 0..width {
                if grid[y * width + x] == b'E' {
                    end.x = x;
                    end.y = y;
                    break;
                }
            }
        }
        ends.push(end);
    }

    // perform BFS
    let mut queue = VecDeque::new();
    queue.push_back(State {
        steps: 0,
        pos: start,
    });
    let mut seen = vec![false; width * height * grids.len()];

    while let Some(State { steps, pos }) = queue.pop_front() {
        if ends.contains(&pos) {
            return steps;
        }

        if let Some(ns) = map.get(&pos) {
            for &np in ns {
                let si = np.y * width * grids.len() + np.x * grids.len() + np.gi;
                if !seen[si] {
                    seen[si] = true;
                    queue.push_back(State {
                        steps: steps + 1,
                        pos: np,
                    });
                }
            }
        }
    }

    unreachable!()
}

fn main() {
    // part 1
    let (width, height, grid) = parse("everybody_codes_e2025_q20_p1.txt");
    println!("{}", count_pairs(&grid, width, height));

    // part 2
    let (width, height, grid) = parse("everybody_codes_e2025_q20_p2.txt");
    let grids = [grid];
    let map = make_map(&grids, width, height, false);
    println!("{}", shortest_path(&grids, width, height, &map));

    // part 3
    let (width, height, grid0) = parse("everybody_codes_e2025_q20_p3.txt");
    let grid1 = rotate(&grid0, width, height);
    let grid2 = rotate(&grid1, width, height);
    let grids = [grid0, grid1, grid2];
    let map = make_map(&grids, width, height, true);
    println!("{}", shortest_path(&grids, width, height, &map));
}
