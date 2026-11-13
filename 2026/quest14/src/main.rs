use std::collections::{HashMap, HashSet};
use std::fs;

// Right, Down, Left, Up
pub const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

struct Node {
    parent: usize,
    size: usize,
}

/// Union–find: find operation
///
/// https://en.wikipedia.org/wiki/Disjoint-set_data_structure
fn find(mut x: usize, nodes: &mut [Node]) -> usize {
    let mut root = x;
    while nodes[root].parent != root {
        root = nodes[root].parent;
    }

    while nodes[x].parent != root {
        let parent = nodes[x].parent;
        nodes[x].parent = root;
        x = parent;
    }

    root
}

/// Union–find: union operation
///
/// https://en.wikipedia.org/wiki/Disjoint-set_data_structure
fn union(mut x: usize, mut y: usize, nodes: &mut [Node]) {
    x = find(x, nodes);
    y = find(y, nodes);

    if x == y {
        return;
    }

    if nodes[x].size < nodes[y].size {
        (x, y) = (y, x);
    }

    nodes[y].parent = x;
    nodes[x].size += nodes[y].size;
}

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

/// Find all unique edges between two pieces in the given grid and count with
/// how many bindings each pair is connected
fn find_unique_edges(grid: Vec<u8>, width: usize, height: usize) -> HashMap<(u8, u8), usize> {
    let mut edges: HashMap<(u8, u8), usize> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let piece = grid[y * width + x];
            for (dx, dy) in DIRS {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                let neighbor =
                    if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                        grid[ny as usize * width + nx as usize]
                    } else {
                        0
                    };

                // only add unique edges
                if neighbor < piece {
                    *edges.entry((neighbor, piece)).or_default() += 1;
                }
            }
        }
    }

    edges
}

/// Compute Minimum Spanning Tree using Kruskal's
///
/// See: https://en.wikipedia.org/wiki/Kruskal's_algorithm
fn mst(edges: HashMap<(u8, u8), usize>) -> usize {
    let mut nodes = (0..255)
        .map(|b| Node { parent: b, size: 1 })
        .collect::<Vec<_>>();

    let mut edges = edges.into_iter().collect::<Vec<_>>();
    edges.sort_unstable_by_key(|e| e.1);

    let mut result = 0;
    for e in edges {
        let f0 = find(e.0.0 as usize, &mut nodes);
        let f1 = find(e.0.1 as usize, &mut nodes);
        if f0 != f1 {
            result += e.1;
            union(f0, f1, &mut nodes);
        }
    }

    result
}

fn main() {
    // part 1 - count the number of unique edges between pieces (including the
    // frame)
    let (width, height, grid) = parse_grid("everybody_codes_e2026_q14_p1.txt");
    let mut seen = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            for (dx, dy) in DIRS {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if (nx < 0 || ny < 0 || (nx as usize) >= width || (ny as usize) >= height)
                    || grid[ny as usize * width + nx as usize] != grid[y * width + x]
                {
                    let mut p1 = (x, y);
                    let mut p2 = (nx as usize, ny as usize);
                    if p2 < p1 {
                        // make pair unique
                        (p1, p2) = (p2, p1);
                    }
                    seen.insert((p1, p2));
                }
            }
        }
    }
    println!("{}", seen.len());

    // part 2 - Minimum Spanning Tree
    let (width, height, grid) = parse_grid("everybody_codes_e2026_q14_p2.txt");
    let edges = find_unique_edges(grid, width, height);
    println!("{}", mst(edges));

    // part 3 - iterate over each pair of pieces A and B, remove A and B, and
    // connect all their neighbors to each other using a 0-weight edge
    let (width, height, grid) = parse_grid("everybody_codes_e2026_q14_p3.txt");
    let edges = find_unique_edges(grid, width, height);

    let mut neighbors: Vec<HashSet<u8>> = vec![HashSet::new(); 255];
    for e in &edges {
        neighbors[e.0.0 as usize].insert(e.0.1);
        neighbors[e.0.1 as usize].insert(e.0.0);
    }
    let all_pieces = (1u8..255)
        .filter(|i| !neighbors[*i as usize].is_empty())
        .collect::<Vec<_>>();

    let mut min = usize::MAX;
    for a in 0..all_pieces.len() {
        for b in a + 1..all_pieces.len() {
            let a = all_pieces[a];
            let b = all_pieces[b];

            let mut edges = edges.clone();

            // pretend `a` and `b` did not exist by replacing all their
            // connections to their neighbors with 0-weight edges
            for &n in &neighbors[a as usize] {
                if n < a {
                    edges.insert((n, a), 0);
                } else {
                    edges.insert((a, n), 0);
                }
            }
            for &n in &neighbors[b as usize] {
                if n < b {
                    edges.insert((n, b), 0);
                } else {
                    edges.insert((b, n), 0);
                }
            }

            // compute Minimum Spanning Tree
            min = min.min(mst(edges));
        }
    }
    println!("{min}");
}
