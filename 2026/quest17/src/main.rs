use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

type Point = (i32, i32);
type Connection = (Point, Point);

// Right, Down, Left, Up
const DIRS: [Point; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

// (
//   (dx, dy),
//   (
//     (dx and dy to get to neighbor 1),
//     (point on edge that connects current cell with neighbor 1)
//   ),
//   (
//     (dx and dy to get to neighbor 2),
//     (point on edge that connects current cell with neighbor 2)
//   ),
// )
const DIRS_WITH_NEIGHBORS: [(Point, Connection, Connection); 4] = [
    (DIRS[0], ((0, -1), (1, 0)), ((0, 1), (1, 1))),
    (DIRS[1], ((-1, 0), (0, 1)), ((1, 0), (1, 1))),
    (DIRS[2], ((0, -1), (0, 0)), ((0, 1), (0, 1))),
    (DIRS[3], ((-1, 0), (0, 0)), ((1, 0), (1, 0))),
];

fn parse_grid(path: &str) -> (usize, usize, Vec<u8>) {
    let input = fs::read_to_string(path).expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .into_iter()
        .flat_map(|l| l.bytes())
        .collect::<Vec<_>>();
    (width, height, grid)
}

fn main() {
    // part 1 - simple BFS to find shortest path
    let (width, height, grid) = parse_grid("everybody_codes_e2026_q17_p1.txt");
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    seen.insert((0, 0));
    queue.push_back((0, (0, 0)));
    while let Some((steps, pos)) = queue.pop_front() {
        if pos == (width as i32 - 1, height as i32 - 1) {
            println!("{steps}");
            break;
        }
        for (dx, dy) in DIRS {
            let nx = pos.0 + dx;
            let ny = pos.1 + dy;
            if nx >= 0
                && ny >= 0
                && (nx as usize) < width
                && (ny as usize) < height
                && grid[ny as usize * width + nx as usize] == b'#'
                && seen.insert((nx, ny))
            {
                queue.push_back((steps + 1, (nx, ny)));
            }
        }
    }

    // part 2 - flood fill and count borders
    let (width, height, grid) = parse_grid("everybody_codes_e2026_q17_p2.txt");
    let mut total = 0;
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    seen.insert((0, 0));
    queue.push_back((0, 0));
    while let Some(pos) = queue.pop_front() {
        for (dx, dy) in DIRS {
            let nx = pos.0 + dx;
            let ny = pos.1 + dy;
            if nx >= 0
                && ny >= 0
                && (nx as usize) < width
                && (ny as usize) < height
                && grid[ny as usize * width + nx as usize] == b'#'
            {
                if seen.insert((nx, ny)) {
                    queue.push_back((nx, ny));
                }
            } else {
                // the cell at (nx, ny) was empty so this is a border
                total += 1;
            }
        }
    }
    println!("{total}");

    // part 3
    let (width, height, grid) = parse_grid("everybody_codes_e2026_q17_p3.txt");

    // flood fill and create graph
    let mut edges: HashMap<Connection, HashSet<Connection>> = HashMap::new();
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    seen.insert((0, 0));
    queue.push_back((0, 0));
    while let Some(pos) = queue.pop_front() {
        for ((dx, dy), n1, n2) in DIRS_WITH_NEIGHBORS {
            let nx = pos.0 + dx;
            let ny = pos.1 + dy;
            if nx >= 0
                && ny >= 0
                && (nx as usize) < width
                && (ny as usize) < height
                && grid[ny as usize * width + nx as usize] == b'#'
            {
                if seen.insert((nx, ny)) {
                    queue.push_back((nx, ny));
                }
            } else {
                // the cell at (nx, ny) was empty so we need to add an edge to
                // our graph and connect the current cell with its neighbors
                let neighbor1 = (pos.0 + n1.0.0, pos.1 + n1.0.1);
                let neighbor2 = (pos.0 + n2.0.0, pos.1 + n2.0.1);
                let corner1 = (pos.0 + n1.1.0, pos.1 + n1.1.1);
                let corner2 = (pos.0 + n2.1.0, pos.1 + n2.1.1);

                edges
                    .entry((pos, corner1))
                    .or_default()
                    .insert((pos, corner2));
                edges
                    .entry((pos, corner2))
                    .or_default()
                    .insert((pos, corner1));

                for (n, c) in [(neighbor1, corner1), (neighbor2, corner2)] {
                    if n.0 >= 0
                        && n.1 >= 0
                        && (n.0 as usize) < width
                        && (n.1 as usize) < height
                        && grid[n.1 as usize * width + n.0 as usize] == b'#'
                    {
                        edges.entry((pos, c)).or_default().insert((n, c));
                        edges.entry((n, c)).or_default().insert((pos, c));
                    }
                }
            }
        }
    }

    // simple BFS again to find shortest path
    let mut queue = VecDeque::new();
    queue.push_back((0, ((0, 0), (0, 0))));
    let mut seen: HashSet<Connection> = HashSet::new();
    seen.insert(((0, 0), (0, 0)));
    while let Some((steps, (cell, pos))) = queue.pop_front() {
        if pos == (width as i32, height as i32) {
            println!("{steps}");
            break;
        }
        for &n in edges.get(&(cell, pos)).unwrap() {
            if seen.insert(n) {
                // only increase steps if we've actually changed position and
                // not just cell
                queue.push_back((if pos == n.1 { steps } else { steps + 1 }, n));
            }
        }
    }
}
