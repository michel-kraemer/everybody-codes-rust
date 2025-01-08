use std::collections::VecDeque;
use std::fs;

const SIZE: usize = 128; // Should be enough for your input. If not, increase it a little
const VERT: [(i32, i32, i32); 2] = [(0, 1, 0), (0, -1, 0)];
const CUBE: [(i32, i32, i32); 6] = [
    (0, 1, 0),
    (0, -1, 0),
    (1, 0, 0),
    (-1, 0, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn idx(x: i32, y: i32, z: i32) -> usize {
    ((z + SIZE as i32 / 2) * (SIZE * SIZE) as i32 + y * SIZE as i32 + x + SIZE as i32 / 2) as usize
}

struct Tree {
    segments: Vec<bool>,
    leaves: Vec<(i32, i32, i32)>,
    height: i32,
    n_segments: usize,
}

fn grow(filename: &str) -> Tree {
    let input = fs::read_to_string(filename).expect("Could not read file");

    let mut segments = vec![false; SIZE * SIZE * SIZE];
    let mut leaves: Vec<(i32, i32, i32)> = Vec::new();
    let mut height = 0;
    let mut n_segments = 0;

    for l in input.lines() {
        let is = l.trim().split(",").collect::<Vec<_>>();

        let mut pos = (0, 0, 0);
        for i in is {
            let dir = match &i[0..1] {
                "U" => (0, 1, 0),
                "D" => (0, -1, 0),
                "R" => (1, 0, 0),
                "L" => (-1, 0, 0),
                "F" => (0, 0, 1),
                "B" => (0, 0, -1),
                _ => panic!(),
            };

            let dist = i[1..].parse::<i32>().unwrap();
            for _ in 0..dist {
                pos.0 += dir.0;
                pos.1 += dir.1;
                pos.2 += dir.2;
                if !segments[idx(pos.0, pos.1, pos.2)] {
                    n_segments += 1;
                }
                segments[idx(pos.0, pos.1, pos.2)] = true;
            }

            height = height.max(pos.1);
        }
        leaves.push(pos);
    }

    Tree {
        segments,
        leaves,
        height,
        n_segments,
    }
}

fn main() {
    let tree1 = grow("everybody_codes_e2024_q14_p1.txt");
    println!("{}", tree1.height);

    let tree2 = grow("everybody_codes_e2024_q14_p2.txt");
    println!("{}", tree2.n_segments);

    // Flood-fill the tree from the leaves to the trunk segments and sum up
    // the distances. Since there are less leaves than trunk segments, it's
    // faster to start at the leaves. An alternative would be to start at each
    // trunk segment and sum up the distances to all leaves in one flood fill.
    let tree3 = grow("everybody_codes_e2024_q14_p3.txt");
    let mut trunk = vec![0; tree3.height as usize];
    for l in tree3.leaves {
        let mut seen = vec![false; SIZE * SIZE * SIZE];
        let mut queue = VecDeque::new();
        queue.push_back((l.0, l.1, l.2, 0));
        seen[idx(l.0, l.1, l.2)] = true;
        while let Some((x, y, z, steps)) = queue.pop_front() {
            let on_trunk = x == 0 && z == 0;
            if on_trunk {
                trunk[y as usize] += steps;
            }

            let dirs = if on_trunk {
                // once we're on the trunk, stay there
                VERT.iter()
            } else {
                CUBE.iter()
            };

            for &(dx, dy, dz) in dirs {
                let nx = x + dx;
                let ny = y + dy;
                let nz = z + dz;
                if tree3.segments[idx(nx, ny, nz)] && !seen[idx(nx, ny, nz)] {
                    seen[idx(nx, ny, nz)] = true;
                    queue.push_back((nx, ny, nz, steps + 1));
                }
            }
        }
    }
    println!("{}", trunk.into_iter().filter(|i| *i > 0).min().unwrap());
}
