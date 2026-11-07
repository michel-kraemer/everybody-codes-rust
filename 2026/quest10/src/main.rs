use rustc_hash::FxHashMap;
use std::{
    fs,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone)]
struct Ghost {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    collisions: u64,
}

impl Ghost {
    fn new(x: i32, y: i32, dx: i32, dy: i32) -> Self {
        Self {
            x,
            y,
            dx,
            dy,
            collisions: 0,
        }
    }
}

/// Implement [`Hash`] for [`Ghost`] because we want to put them into a
/// [`HashMap`] to look for cycles. Skip the `collisions` field as this is not
/// relevant for cycle detection.
impl Hash for Ghost {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.dx.hash(state);
        self.dy.hash(state);
    }
}

impl Eq for Ghost {}

/// Implement [`PartialEq`] similarly to [`Hash`], i.e. skip the `collisions`
/// field, too
impl PartialEq for Ghost {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.dx == other.dx && self.dy == other.dy
    }
}

pub fn parse(path: &str) -> (usize, usize, Vec<u8>) {
    let input = fs::read_to_string(path).expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .into_iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();
    (width, height, grid)
}

fn simulate(path: &str, max_rounds: u64, max_collisions: u64) -> (u64, u64) {
    let (width, height, grid) = parse(path);

    // find initial positions and directions of all ghosts
    let mut ghosts = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let c = grid[y * width + x];
            if c == b'<' {
                ghosts.push(Ghost::new(x as i32, y as i32, -1, 0));
            } else if c == b'>' {
                ghosts.push(Ghost::new(x as i32, y as i32, 1, 0));
            } else if c == b'^' {
                ghosts.push(Ghost::new(x as i32, y as i32, 0, -1));
            } else if c == b'v' {
                ghosts.push(Ghost::new(x as i32, y as i32, 0, 1));
            }
        }
    }

    // a cache that allows us to detect cycles (i.e. situations where all ghosts
    // are at the same positions and look into the same directions as before)
    let mut cache: FxHashMap<Vec<Ghost>, u64> = FxHashMap::default();

    let mut collisions = 0;
    let mut round = 1;
    while round <= max_rounds {
        // try to move ghosts
        for g in &mut ghosts {
            let nx = g.x + g.dx;
            let ny = g.y + g.dy;

            // no bounds check necessary because the area is always surrounded
            // by walls
            if grid[ny as usize * width + nx as usize] == b'#' {
                // Ghost hit a wall. Turn it clockwise.
                (g.dx, g.dy) = (-g.dy, g.dx);
                collisions += 1;
                g.collisions += 1;
            } else {
                g.x = nx;
                g.y = ny;
            }
        }

        // Remove all ghosts that reached their limit. Do this before we check
        // for ghost-ghost collisions.
        ghosts.retain(|g| g.collisions < max_collisions);
        if ghosts.is_empty() {
            break;
        }

        // check for collisions between ghosts
        for i in 0..ghosts.len() {
            for j in 0..ghosts.len() {
                if i != j && ghosts[i].x == ghosts[j].x && ghosts[i].y == ghosts[j].y {
                    // Ghost hit another ghost. Turn it by 180 degrees.
                    collisions += 1;
                    ghosts[i].dx = -ghosts[i].dx;
                    ghosts[i].dy = -ghosts[i].dy;
                    ghosts[i].collisions += 1;
                    break;
                }
            }
        }

        // remove all ghosts that reached their limit
        ghosts.retain(|g| g.collisions < max_collisions);
        if ghosts.is_empty() {
            break;
        }

        // check if we've encountered this situation before
        if let Some((prev_ghosts, prev_round)) = cache.get_key_value(&ghosts) {
            // we found a cycle!
            let cycle_len = round - prev_round;

            // compute how many cycles we can skip ahead until the next ghost
            // vanishes
            let mut cycles = 0;
            let mut next_round = u64::MAX;
            for i in 0..ghosts.len() {
                // compute how many cycles this ghost has left before it
                // vanishes
                let remaining_cycles = (max_collisions - ghosts[i].collisions)
                    .div_ceil(ghosts[i].collisions - prev_ghosts[i].collisions)
                    - 1;

                // compute the round after which the ghost will vanish within
                // at most one cycle
                let nr = round + remaining_cycles * cycle_len;

                if nr < next_round {
                    next_round = nr;
                    cycles = remaining_cycles;
                }
            }

            // skip ahead until the round after which at least one ghost will
            // vanish within at most one cycle
            round = next_round;

            // update collisions of all ghosts accordingly
            for i in 0..ghosts.len() {
                let inc = ghosts[i].collisions - prev_ghosts[i].collisions;
                ghosts[i].collisions += inc * cycles;
            }

            cache.clear();
        } else {
            cache.insert(ghosts.clone(), round);
        }

        round += 1;
    }

    (collisions, round)
}

fn main() {
    // part 1
    let (collisions, _) = simulate("everybody_codes_e2026_q10_p1.txt", 10, u64::MAX);
    println!("{collisions}");

    // part 2
    let (_, round) = simulate("everybody_codes_e2026_q10_p2.txt", u64::MAX, 10000);
    println!("{round}");

    // part 3
    let (_, round) = simulate(
        "everybody_codes_e2026_q10_p3.txt",
        u64::MAX,
        1_000_000_000_000u64,
    );
    println!("{round}");
}
