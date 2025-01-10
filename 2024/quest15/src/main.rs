use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Node {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq)]
struct State {
    steps: usize,
    node: Node,
    id: usize,
    remaining: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.cmp(&other.steps).then(
            self.remaining
                .count_ones()
                .cmp(&other.remaining.count_ones()),
        )
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs(
    s: Node,
    grid: &[u8],
    width: usize,
    height: usize,
    herb_ids: &HashMap<u8, usize>,
    net: &mut HashMap<Node, Vec<(Node, usize, usize)>>,
) {
    let o = grid[s.y * width + s.x];

    let mut queue: VecDeque<(Node, usize)> = VecDeque::new();
    queue.push_back((s, 0));

    let mut seen = HashSet::new();
    seen.insert(s);

    while let Some((node, steps)) = queue.pop_front() {
        let c = grid[node.y * width + node.x];
        if (c.is_ascii_alphabetic() || c == b'@') && c != o {
            let cid = *herb_ids.get(&c).unwrap();
            net.entry(s).or_default().push((node, cid, steps));

            // If we've encountered a node, don't walk through it. This will
            // prevent us from having too many edges. The input file for part 3
            // is designed in a way that divides it into three sections that
            // are connected via single nodes.
            continue;
        }

        for (dx, dy) in DIRS {
            let nx = node.x as i32 + dx;
            let ny = node.y as i32 + dy;
            let nn = Node {
                x: nx as usize,
                y: ny as usize,
            };
            if nx >= 0
                && ny >= 0
                && nx < width as i32
                && ny < height as i32
                && grid[ny as usize * width + nx as usize] != b'#'
                && grid[ny as usize * width + nx as usize] != b'~'
                && !seen.contains(&nn)
            {
                seen.insert(nn);
                queue.push_back((nn, steps + 1));
            }
        }
    }
}

fn main() {
    for part in [1, 2, 3] {
        let input = fs::read_to_string(format!("everybody_codes_e2024_q15_p{}.txt", part))
            .expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();
        let mut grid = lines
            .into_iter()
            .flat_map(|l| l.as_bytes().iter().copied())
            .collect::<Vec<_>>();

        // look for start
        let mut start = Node { x: 0, y: 0 };
        for (x, &c) in grid.iter().enumerate().take(width) {
            if c == b'.' {
                start = Node { x, y: 0 };
                break;
            }
        }

        // insert artificial start node
        grid[start.y * width + start.x] = b'@';

        let mut nodes = Vec::new();
        let mut herb_ids = HashMap::new();
        let mut all_herbs: u32 = 0;

        herb_ids.insert(b'@', 0);
        nodes.push(start);

        // detect all herbs
        for y in 0..height {
            for x in 0..width {
                let c = grid[y * width + x];
                if c.is_ascii_alphabetic() {
                    nodes.push(Node { x, y });
                    if !herb_ids.contains_key(&c) {
                        let l = herb_ids.len();
                        herb_ids.insert(c, l);
                        all_herbs |= 1 << l;
                    }
                }
            }
        }

        // for each herb (and the start node), perform a BFS to find the
        // shortest path to all its direct neighbors
        let mut net = HashMap::new();
        for n in nodes {
            bfs(n, &grid, width, height, &herb_ids, &mut net);
        }

        // perform Dijkstra to find the shortest path between all nodes and
        // make sure we collect all types of herbs
        let mut dists: HashMap<(Node, u32), usize> = HashMap::new();
        let mut queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();
        queue.push(Reverse(State {
            steps: 0,
            node: start,
            id: 0,
            remaining: all_herbs,
        }));

        while let Some(Reverse(State {
            steps,
            node,
            id,
            remaining,
        })) = queue.pop()
        {
            if remaining == 0 && id == 0 {
                println!("{}", steps);
                break;
            }

            let new_remaining = remaining & !(1 << id);
            for n in net.get(&node).unwrap() {
                let new_steps = steps + n.2;
                if (n.1 != 0
                    && *dists.get(&(n.0, new_remaining)).unwrap_or(&usize::MAX) > new_steps)
                    || (n.1 == 0 && new_remaining == 0)
                {
                    dists.insert((n.0, new_remaining), new_steps);
                    queue.push(Reverse(State {
                        steps: new_steps,
                        node: n.0,
                        id: n.1,
                        remaining: new_remaining,
                    }));
                }
            }
        }
    }
}
