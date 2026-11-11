use std::fs;

// Right, Down, Left, Up
pub const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

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

/// Recursively remove largest horizontal or vertical rectangles from a single
/// shape in the given grid and return the minimum number of rays needed to
/// reflect the whole shape
fn reflect_shape(grid: Vec<u8>, width: usize, height: usize) -> u64 {
    // find next rectangle
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] != b'#' {
                continue;
            }

            // find largest horizontal rectangle, remove it, and recurse
            let r1 = {
                let min_x = x;
                let mut max_x = x;
                while max_x + 1 < width && grid[y * width + max_x + 1] == b'#' {
                    max_x += 1;
                }

                let min_y = y;
                let mut max_y = y;
                while max_y + 1 < height
                    && (min_x..=max_x).all(|x2| grid[(max_y + 1) * width + x2] == b'#')
                    && (min_x == 0 || grid[(max_y + 1) * width + min_x - 1] != b'#')
                    && (max_x == width - 1 || grid[(max_y + 1) * width + max_x + 1] != b'#')
                {
                    max_y += 1;
                }

                let mut grid2 = grid.clone();
                for y2 in min_y..=max_y {
                    for x2 in min_x..=max_x {
                        grid2[y2 * width + x2] = b'.';
                    }
                }

                let w = max_x - min_x + 1;
                let h = max_y - min_y + 1;

                w.min(h) as u64 + reflect_shape(grid2, width, height)
            };

            // find largest vertical rectangle, remove it, and recurse
            let r2 = {
                let min_y = y;
                let mut max_y = y;
                while max_y + 1 < height && grid[(max_y + 1) * width + x] == b'#' {
                    max_y += 1;
                }

                let min_x = x;
                let mut max_x = x;
                while max_x + 1 < width
                    && (min_y..=max_y).all(|y2| grid[y2 * width + max_x + 1] == b'#')
                    && (min_y == 0 || grid[(min_y - 1) * width + max_x + 1] != b'#')
                    && (max_y == height - 1 || grid[(max_y + 1) * width + max_x + 1] != b'#')
                {
                    max_x += 1;
                }

                let mut grid2 = grid.clone();
                for y2 in min_y..=max_y {
                    for x2 in min_x..=max_x {
                        grid2[y2 * width + x2] = b'.';
                    }
                }

                let w = max_x - min_x + 1;
                let h = max_y - min_y + 1;

                w.min(h) as u64 + reflect_shape(grid2, width, height)
            };

            return r1.min(r2);
        }
    }

    // no more rectangles to find
    0
}

/// Find individual shapes in the given grid and return the sum of the minimum
/// number of rays needed to reflect each shape
fn reflect(mut grid: Vec<u8>, width: usize, height: usize) -> u64 {
    let mut total = 0;

    // find next shape
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] != b'#' {
                continue;
            }

            // Flood-fill the shape and collect all coordinates. Erase it from
            // the grid while doing so.
            let mut coords = Vec::new();
            let mut min_x = x;
            let mut min_y = y;
            let mut max_x = x;
            let mut max_y = y;
            grid[y * width + x] = b'.';
            let mut queue = vec![(x, y)];
            while let Some((x, y)) = queue.pop() {
                coords.push((x, y));
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                for (dx, dy) in DIRS {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0
                        && (nx as usize) < width
                        && ny >= 0
                        && (ny as usize) < height
                        && grid[ny as usize * width + nx as usize] == b'#'
                    {
                        grid[ny as usize * width + nx as usize] = b'.';
                        queue.push((nx as usize, ny as usize));
                    }
                }
            }

            // create a small grid that only contains the shape we've just
            // extracted
            let w = max_x - min_x + 1;
            let h = max_y - min_y + 1;
            let mut grid2 = vec![b'.'; w * h];
            for (x2, y2) in coords {
                grid2[(y2 - min_y) * w + x2 - min_x] = b'#';
            }

            // reflect the shape with as few rays as possible
            total += reflect_shape(grid2, w, h);
        }
    }

    total
}

fn main() {
    // part 1
    let (width, height, grid) = parse("everybody_codes_e2026_q12_p1.txt");
    println!("{}", reflect(grid, width, height));

    // part 2
    let (width, height, grid) = parse("everybody_codes_e2026_q12_p2.txt");
    println!("{}", reflect(grid, width, height));

    // part 3
    let (width, height, grid) = parse("everybody_codes_e2026_q12_p3.txt");
    println!("{}", reflect(grid, width, height));
}
