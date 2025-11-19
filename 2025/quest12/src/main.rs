use std::collections::{HashSet, VecDeque};
use std::fs;

/// A node in a union-find data structure
struct Node {
    /// The node's parent
    parent: usize,

    /// The set's size
    size: usize,

    /// How many barrels will explode if we set this set on fire
    can_explode: usize,

    /// The set's children (IDs of other sets)
    children: HashSet<usize>,
}

/// Find a node's parent
fn find(x: usize, nodes: &mut [Node]) -> usize {
    if nodes[x].parent != x {
        nodes[x].parent = find(nodes[x].parent, nodes);
        nodes[x].parent
    } else {
        x
    }
}

/// Merge two sets
fn union(mut x: usize, mut y: usize, nodes: &mut [Node]) {
    x = find(x, nodes);
    y = find(y, nodes);

    if x == y {
        return;
    }

    if nodes[x].size < nodes[y].size {
        std::mem::swap(&mut x, &mut y);
    }

    nodes[y].parent = x;
    nodes[x].size += nodes[y].size;
    nodes[x].can_explode = nodes[x].size;
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

/// Count how many values of the given slice are true
fn count_seen(s: &[bool]) -> usize {
    s.iter().filter(|v| **v).count()
}

/// Set a barrel on fire. Simple BFS.
fn set_on_fire(grid: &[u8], width: usize, height: usize, start: (usize, usize)) -> Vec<bool> {
    let mut queue = VecDeque::new();
    let mut seen = vec![false; width * height];

    queue.push_back(start);
    seen[start.1 * width + start.0] = true;

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && (nx as usize) < width
                && ny >= 0
                && (ny as usize) < height
                && grid[ny as usize * width + nx as usize] <= grid[y * width + x]
                && !seen[ny as usize * width + nx as usize]
            {
                seen[ny as usize * width + nx as usize] = true;
                queue.push_back((nx as usize, ny as usize));
            }
        }
    }

    seen
}

/// Merge `other` into `s`
fn merge(s: &mut [bool], other: &[bool]) {
    for (i, c) in s.iter_mut().enumerate() {
        if other[i] {
            *c = true;
        }
    }
}

/// Find the set with the largest number of barrels that would explode if we
/// set one item of this set on fire. For each set, starting with the one with
/// the lowest barrel value, find the neighbors with a larger barrel value and
/// add the set's size and the size of its unique children to the neighbor's
/// `can_explode` value. Since `indices` is sorted from lowest barrel value to
/// highest, this accumulates the `can_explode` values up to the one set with
/// the highest value.
fn find_max(
    grid: &[u8],
    width: usize,
    height: usize,
    indices: &[(u8, (usize, usize))],
    nodes: &mut [Node],
) -> usize {
    let mut max = 0;
    let mut max_node = 0;

    for &(_, (x, y)) in indices {
        let node = find(y * width + x, nodes);
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && (nx as usize) < width
                && ny >= 0
                && (ny as usize) < height
                && grid[ny as usize * width + nx as usize] > grid[y * width + x]
            {
                let neighbor = find(ny as usize * width + nx as usize, nodes);
                if !nodes[neighbor].children.contains(&node) {
                    nodes[neighbor].children.insert(node);
                    nodes[neighbor].can_explode += nodes[node].size;
                    let children_to_insert = nodes[node]
                        .children
                        .iter()
                        .filter(|c| !nodes[neighbor].children.contains(c))
                        .copied()
                        .collect::<Vec<_>>();
                    for c in children_to_insert {
                        nodes[neighbor].children.insert(c);
                        nodes[neighbor].can_explode += nodes[c].size;
                    }
                    if nodes[neighbor].can_explode > max {
                        max = nodes[neighbor].can_explode;
                        max_node = neighbor;
                    }
                }
            }
        }
    }

    max_node
}

fn main() {
    // part 1
    let (width, height, grid) = parse("everybody_codes_e2025_q12_p1.txt");
    println!("{}", count_seen(&set_on_fire(&grid, width, height, (0, 0))));

    // part 2
    let (width, height, grid) = parse("everybody_codes_e2025_q12_p2.txt");
    let mut s = set_on_fire(&grid, width, height, (0, 0));
    merge(
        &mut s,
        &set_on_fire(&grid, width, height, (width - 1, height - 1)),
    );
    println!("{}", count_seen(&s));

    // part 3 ...
    let (width, height, grid) = parse("everybody_codes_e2025_q12_p3.txt");

    // prepare nodes for union-find data structure
    let mut nodes = Vec::new();
    let mut indices = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let i = nodes.len();
            let v = grid[y * width + x];
            nodes.push(Node {
                parent: i,
                size: 1,
                can_explode: 1,
                children: HashSet::new(),
            });
            indices.push((v, (x, y)));
        }
    }

    // find all connected components
    for y in 0..height {
        for x in 0..width {
            let node = find(y * width + x, &mut nodes);
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0
                    && (nx as usize) < width
                    && ny >= 0
                    && (ny as usize) < height
                    && grid[ny as usize * width + nx as usize] == grid[y * width + x]
                {
                    let neighbor = find(ny as usize * width + nx as usize, &mut nodes);
                    union(node, neighbor, &mut nodes);
                }
            }
        }
    }

    // sort connected components by their barrel value
    indices.sort_unstable_by_key(|n| n.0);

    // perform three rounds ...
    let mut total = 0;
    for round in 0..3 {
        // find the set with the highest `can_explode` value
        let max_node = find_max(&grid, width, height, &indices, &mut nodes);
        total += nodes[max_node].can_explode;
        if round == 2 {
            break;
        }

        // prepare next round ...
        // merge all children of max_node into one set
        for c in nodes[max_node].children.clone() {
            union(c, max_node, &mut nodes);
        }
        let max_node = find(max_node, &mut nodes);

        // remove all grid cells of max_node from indices, so we don't visit
        // them anymore
        indices = indices
            .into_iter()
            .filter(|i| {
                let n = find(i.1.1 * width + i.1.0, &mut nodes);
                n != max_node
            })
            .collect::<Vec<_>>();

        // reset state for the next round
        for i in 0..nodes.len() {
            let n = find(i, &mut nodes);
            nodes[n].children.clear();
            nodes[n].can_explode = nodes[n].size;
        }
    }

    println!("{total}");
}
