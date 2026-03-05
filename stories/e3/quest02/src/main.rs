use std::fs;

// Right, Down, Left, Up
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const CLOCKWISE: [(i32, i32); 8] = [
    (1, 0),   // →
    (1, 1),   // ↘︎
    (0, 1),   // ↓
    (-1, 1),  // ↙︎
    (-1, 0),  // ←
    (-1, -1), // ↖︎
    (0, -1),  // ↑
    (1, -1),  // ↗︎
];

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<u8>,
}

impl Grid {
    /// Parse a file and create a new Grid
    fn new(path: &str) -> Self {
        let input = fs::read_to_string(path).expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();
        Self {
            width: lines[0].len(),
            height: lines.len(),
            grid: lines.iter().flat_map(|l| l.as_bytes()).copied().collect(),
        }
    }

    /// Check if the given coordinates are within the grid's bounds
    fn has(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width() && y >= 0 && y < self.height()
    }

    /// Get the grid's width
    fn width(&self) -> i32 {
        self.width as i32
    }

    /// Get the grid's height
    fn height(&self) -> i32 {
        self.height as i32
    }

    /// Get the value of the grid cell at the given coordinates
    fn get(&self, x: i32, y: i32) -> u8 {
        self.grid[y as usize * self.width + x as usize]
    }

    /// Set the value of the grid cell at the given coordinates
    fn set(&mut self, x: i32, y: i32, c: u8) {
        self.grid[y as usize * self.width + x as usize] = c;
    }

    /// Extend the grid one column to the left
    fn extend_left(&mut self) {
        let mut new_grid = Vec::with_capacity((self.width + 1) * self.height);
        let mut i = 0;
        for _ in 0..self.height {
            new_grid.push(b'.');
            new_grid.extend(&self.grid[i..i + self.width]);
            i += self.width;
        }
        self.grid = new_grid;
        self.width += 1;
    }

    /// Extend the grid one column to the right
    fn extend_right(&mut self) {
        let mut new_grid = Vec::with_capacity((self.width + 1) * self.height);
        let mut i = 0;
        for _ in 0..self.height {
            new_grid.extend(&self.grid[i..i + self.width]);
            new_grid.push(b'.');
            i += self.width;
        }
        self.grid = new_grid;
        self.width += 1;
    }

    /// Extend the grid one row to the top
    fn extend_top(&mut self) {
        let mut new_grid = vec![b'.'; self.width];
        new_grid.extend(&self.grid);
        self.grid = new_grid;
        self.height += 1;
    }

    /// Extend the grid one row to the bottom
    fn extend_bottom(&mut self) {
        self.grid.extend(std::iter::repeat_n(b'.', self.width));
        self.height += 1;
    }

    /// Flood-fill all empty grid cells starting at the given position with the
    /// given value
    fn fill(&mut self, start_x: i32, start_y: i32, c: u8) {
        if self.get(start_x, start_y) != b'.' {
            return;
        }
        let mut queue = vec![(start_x, start_y)];
        while let Some((x, y)) = queue.pop() {
            self.set(x, y, c);
            for (dx, dy) in DIRS {
                let nx = x + dx;
                let ny = y + dy;
                if self.has(nx, ny) && self.get(nx, ny) == b'.' {
                    queue.push((nx, ny));
                }
            }
        }
    }
}

/// Find the positions of all bones in a grid
fn find_bones(grid: &Grid) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(x, y) == b'#' {
                result.push((x, y));
            }
        }
    }
    result
}

/// Check if all bones in the grid are surrounded
fn all_bones_surrounded(mut bones: Vec<(i32, i32)>) -> impl FnMut(&Grid, i32, i32) -> bool {
    move |grid, offset_x, offset_y| -> bool {
        bones.retain(|b| {
            for (dx, dy) in DIRS {
                let nx = b.0 + offset_x + dx;
                let ny = b.1 + offset_y + dy;
                if !grid.has(nx, ny) || grid.get(nx, ny) == b'.' {
                    return true;
                }
            }
            false
        });
        bones.is_empty()
    }
}

/// Apply the main algorithm to a grid using the given instructions. Stop as
/// soon as the given predicate returns `true`.
fn run<P>(mut grid: Grid, instructions: &[(i32, i32)], mut predicate: P) -> u64
where
    P: FnMut(&Grid, i32, i32) -> bool,
{
    let mut instructions = instructions.iter().cycle().peekable();

    // find start position
    let mut pos = (0, 0);
    'outer: for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(x, y) == b'@' {
                pos = (x, y);
                break 'outer;
            }
        }
    }

    let mut steps = 0;
    let mut offset_x = 0;
    let mut offset_y = 0;
    loop {
        let ins = instructions.next().unwrap();
        let mut nx = pos.0 + ins.0;
        let mut ny = pos.1 + ins.1;

        // extend grid if necessary
        if nx < 0 {
            grid.extend_left();
            nx += 1;
            offset_x += 1;
        } else if ny < 0 {
            grid.extend_top();
            ny += 1;
            offset_y += 1;
        } else if nx == grid.width() {
            grid.extend_right();
        } else if ny == grid.height() {
            grid.extend_bottom();
        } else if grid.get(nx, ny) != b'.' {
            // instruction does not work
            while *instructions.peek().unwrap() == ins {
                instructions.next();
            }
            continue;
        }

        // take step
        steps += 1;
        grid.set(nx, ny, b'+');

        // check if we need to fill
        let mut needs_fill = false;
        for (dx, dy) in CLOCKWISE {
            let ox = nx + dx;
            let oy = ny + dy;
            if ox == pos.0 && oy == pos.1 {
                continue;
            }
            if grid.has(ox, oy) && grid.get(ox, oy) != b'.' {
                needs_fill = true;
                break;
            }
        }

        pos = (nx, ny);

        if steps == 1 || needs_fill {
            // flood-fill grid from all border cells
            let width = grid.width();
            let height = grid.height();
            for x in 0..width {
                grid.fill(x, 0, b'F');
                grid.fill(x, height - 1, b'F');
            }
            for y in 0..height {
                grid.fill(0, y, b'F');
                grid.fill(width - 1, y, b'F');
            }

            // clear all filled cells again and instead fill those that could
            // not be reached from any border cell
            for c in grid.grid.iter_mut() {
                if *c == b'.' {
                    *c = b'+';
                } else if *c == b'F' {
                    *c = b'.';
                }
            }
        }

        if predicate(&grid, offset_x, offset_y) {
            break;
        }
    }

    steps
}

fn main() {
    // part 1
    let mut grid = Grid::new("everybody_codes_e3_q02_p1.txt");
    let instructions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dest = (0, 0);
    'outer: for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(x, y) == b'#' {
                dest = (x, y);
                grid.set(x, y, b'.');
                break 'outer;
            }
        }
    }
    println!(
        "{}",
        run(grid, &instructions, |g, offset_x, offset_y| g
            .get(dest.0 + offset_x, dest.1 + offset_y)
            == b'+')
    );

    // part 2
    let grid = Grid::new("everybody_codes_e3_q02_p2.txt");
    let bones = find_bones(&grid);
    println!("{}", run(grid, &instructions, all_bones_surrounded(bones)));

    // part 3
    let grid = Grid::new("everybody_codes_e3_q02_p3.txt");
    let instructions = [
        (0, -1),
        (0, -1),
        (0, -1),
        (1, 0),
        (1, 0),
        (1, 0),
        (0, 1),
        (0, 1),
        (0, 1),
        (-1, 0),
        (-1, 0),
        (-1, 0),
    ];
    let bones = find_bones(&grid);
    println!("{}", run(grid, &instructions, all_bones_surrounded(bones)));
}
