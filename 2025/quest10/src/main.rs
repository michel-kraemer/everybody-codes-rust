use std::fs;

use rustc_hash::FxHashMap;

/// The moves a dragon can make
const KNIGHT_MOVES: [(i64, i64); 8] = [
    (2, 1),
    (2, -1),
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
    (-2, 1),
    (-2, -1),
];

/// A binary mask of sheep positions. Each sheep position is stored in 4 bits,
/// allowing for up to 8 sheep on a grid of a height up to 14. A value of 0xf
/// indicates there is no sheep in that column.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct SheepMask(u32);

impl SheepMask {
    /// Create an empty sheep mask
    fn new() -> Self {
        Self(!0)
    }

    /// Get the y position of the sheep in column x, or 0xf if there is no sheep
    fn get(&self, x: i64) -> i64 {
        ((self.0 >> (x * 4)) & 0xf) as i64
    }

    /// Set the y position of the sheep in column x to y
    fn set(&self, x: i64, y: i64) -> Self {
        let mut r = self.0 & !(0xf << (x * 4));
        r |= (y as u32 & 0xf) << (x * 4);
        Self(r)
    }

    /// Remove the sheep in column x
    fn clear(&self, x: i64) -> Self {
        self.set(x, 0xf)
    }

    /// Check if there are no sheep left
    fn is_empty(&self) -> bool {
        self.0 == !0
    }
}

/// Parse the input file and return a tuple with the grid's width, height, and
/// the grid itself
fn parse(path: &str) -> (i64, i64, Vec<u8>) {
    let input = fs::read_to_string(path).expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();
    (width as i64, height as i64, grid)
}

/// Move dragons. If `stay` is true, dragons will be cloned and the clone will
/// remain at the original position.
fn move_dragons(grid: Vec<u8>, width: i64, height: i64, stay: bool) -> Vec<u8> {
    let mut new_grid = if stay {
        grid.to_vec()
    } else {
        vec![b'.'; grid.len()]
    };
    for y in 0..height {
        for x in 0..width {
            if grid[(y * width + x) as usize] == b'D' {
                for m in &KNIGHT_MOVES {
                    let nx = x + m.0;
                    let ny = y + m.1;
                    if nx >= 0 && nx < width && ny >= 0 && ny < height {
                        new_grid[(ny * width + nx) as usize] = b'D';
                    }
                }
            }
        }
    }
    new_grid
}

/// Move all sheep one row down
fn move_sheep(grid: Vec<u8>, width: i64, height: i64) -> Vec<u8> {
    let mut new_grid = vec![b'.'; grid.len()];
    for y in 0..height - 1 {
        for x in 0..width {
            if grid[(y * width + x) as usize] == b'S' {
                new_grid[((y + 1) * width + x) as usize] = b'S';
            }
        }
    }
    new_grid
}

/// Remove all sheep that are on the same cell as a dragon and not in a hideout.
/// Return the number of removed sheep.
fn remove_sheep(sheep: &mut [u8], dragons: &[u8], grid: &[u8], width: i64, height: i64) -> u64 {
    let mut result = 0;
    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) as usize;
            if sheep[i] == b'S' && dragons[i] == b'D' && grid[i] != b'#' {
                result += 1;
                sheep[i] = b'.';
            }
        }
    }
    result
}

fn dfs(
    state: SheepMask,
    dragon: (i64, i64),
    hideouts: &[u8],
    width: i64,
    height: i64,
    cache: &mut FxHashMap<(SheepMask, (i64, i64)), u64>,
) -> u64 {
    if let Some(c) = cache.get(&(state, dragon)) {
        return *c;
    }

    // try to move sheep
    let mut result = 0;
    let mut moved = 0;
    for x in 0..width {
        let y = state.get(x);
        if y != 0xf {
            if y + 1 == height || hideouts[((y + 1) * width + x) as usize] == b'H' {
                // sheep has escaped - this state can be discarded
                moved += 1;
                continue;
            } else if dragon != (x, y + 1) || hideouts[((y + 1) * width + x) as usize] == b'#' {
                // move sheep one step down and then continue with moving the
                // dragon
                result += dfs_dragon(state.set(x, y + 1), dragon, hideouts, width, height, cache);
                moved += 1;
            }
        }
    }

    if moved == 0 {
        // no sheep could be moved - we need to move the dragon anyhow
        result += dfs_dragon(state, dragon, hideouts, width, height, cache);
    }

    cache.insert((state, dragon), result);

    result
}

fn dfs_dragon(
    state: SheepMask,
    dragon: (i64, i64),
    hideouts: &[u8],
    width: i64,
    height: i64,
    cache: &mut FxHashMap<(SheepMask, (i64, i64)), u64>,
) -> u64 {
    let mut result = 0;

    for d in &KNIGHT_MOVES {
        let nx = dragon.0 + d.0;
        let ny = dragon.1 + d.1;
        if nx < 0 || nx >= width || ny < 0 || ny >= height {
            // dragon would land outside of grid
            continue;
        }

        let mut new_state = state;
        if new_state.get(nx) == ny && hideouts[(ny * width + nx) as usize] != b'#' {
            // dragon eats sheep
            new_state = new_state.clear(nx);
            if new_state.is_empty() {
                // all sheep have been eaten
                result += 1;
                continue;
            }
        }

        result += dfs(new_state, (nx, ny), hideouts, width, height, cache);
    }

    result
}

fn main() {
    // part 1
    let (width, height, grid) = parse("everybody_codes_e2025_q10_p1.txt");
    let mut dragons = grid.clone();
    for _ in 0..4 {
        dragons = move_dragons(dragons, width, height, true);
    }
    let total = remove_sheep(&mut grid.clone(), &dragons, &grid, width, height);
    println!("{total}");

    // part 2
    let (width, height, grid) = parse("everybody_codes_e2025_q10_p2.txt");
    let mut dragons = grid.clone();
    let mut sheep = grid.clone();
    let mut total = 0;
    for _ in 0..20 {
        dragons = move_dragons(dragons, width, height, false);
        total += remove_sheep(&mut sheep, &dragons, &grid, width, height);
        sheep = move_sheep(sheep, width, height);
        total += remove_sheep(&mut sheep, &dragons, &grid, width, height);
    }
    println!("{total}");

    // part 3
    let (width, height, mut grid) = parse("everybody_codes_e2025_q10_p3.txt");

    let mut dragon = (0, 0);
    let mut sheep = SheepMask::new();
    let mut cache = FxHashMap::default();

    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) as usize;
            if grid[i] == b'D' {
                dragon = (x, y);
            } else if grid[i] == b'S' {
                sheep = sheep.set(x, y);
            }
        }
    }

    // mark hideouts that stretch to the bottom as such
    for x in 0..width {
        let mut y = height - 1;
        while y >= 0 && grid[(y * width + x) as usize] == b'#' {
            grid[(y * width + x) as usize] = b'H';
            y -= 1;
        }
    }

    println!("{}", dfs(sheep, dragon, &grid, width, height, &mut cache));
}
