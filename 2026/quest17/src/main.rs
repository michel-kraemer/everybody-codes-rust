use std::collections::VecDeque;
use std::fs;

type Point = (i32, i32);

// Right, Down, Left, Up
const DIRS: [Point; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const DIRS_WITH_EDGES: [(Point, (usize, usize)); 4] = [
    ((1, 0), (2, 1)),
    ((0, 1), (1, 2)),
    ((-1, 0), (0, 1)),
    ((0, -1), (1, 0)),
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
    let mut seen = vec![false; grid.len()];
    seen[0] = true;
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
                && !seen[ny as usize * width + nx as usize]
            {
                seen[ny as usize * width + nx as usize] = true;
                queue.push_back((steps + 1, (nx, ny)));
            }
        }
    }

    // part 2 - flood fill and count borders
    let (width, height, grid) = parse_grid("everybody_codes_e2026_q17_p2.txt");
    let mut total = 0;
    let mut queue = VecDeque::new();
    let mut seen = vec![false; grid.len()];
    seen[0] = true;
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
                if !seen[ny as usize * width + nx as usize] {
                    seen[ny as usize * width + nx as usize] = true;
                    queue.push_back((nx, ny));
                }
            } else {
                // the cell at (nx, ny) was empty so this is a border
                total += 1;
            }
        }
    }
    println!("{total}");

    // part 3 ...
    // Blow up the grid by 3 and place corner points '#' and edge points 'O'
    //
    // Examples:
    //
    //           #O#
    //  #    =>  O O
    //           #O#
    //
    //           #O##O#
    // ##    =>  O    O
    //           #O##O#
    //
    //           #O##O#
    //           O    O
    // ##    =>  #O## #
    //  #           # #
    //              O O
    //              #O#
    //
    //           #O#
    //           O O
    // #     =>  #O#
    //  #           #O#
    //              O O
    //              #O#
    let (width, height, grid) = parse_grid("everybody_codes_e2026_q17_p3.txt");
    let exwidth = width * 3;
    let exheight = height * 3;
    let mut exgrid = vec![b'.'; exwidth * exheight];
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'#' {
                // place four corner points
                exgrid[(y * 3) * exwidth + (x * 3)] = b'#';
                exgrid[(y * 3 + 2) * exwidth + (x * 3)] = b'#';
                exgrid[(y * 3) * exwidth + (x * 3 + 2)] = b'#';
                exgrid[(y * 3 + 2) * exwidth + (x * 3 + 2)] = b'#';

                for ((dx, dy), (ex, ey)) in DIRS_WITH_EDGES {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if !(nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height)
                        || grid[ny as usize * width + nx as usize] == b'.'
                    {
                        // The neighboring cell in the original grid is empty.
                        // Place an edge point.
                        exgrid[(y * 3 + ey) * exwidth + (x * 3 + ex)] = b'O';
                    }
                }
            }
        }
    }

    // perform BFS to find the shortest path but only count edge points 'O'
    let mut queue = VecDeque::new();
    queue.push_back((0, (0, 0)));
    let mut seen = vec![false; exgrid.len()];
    seen[0] = true;
    while let Some((steps, pos)) = queue.pop_front() {
        if pos == (exwidth as i32 - 1, exheight as i32 - 1) {
            println!("{steps}");
            break;
        }
        for (dx, dy) in DIRS {
            let nx = pos.0 + dx;
            let ny = pos.1 + dy;
            if nx >= 0
                && ny >= 0
                && (nx as usize) < exwidth
                && (ny as usize) < exheight
                && exgrid[ny as usize * exwidth + nx as usize] != b'.'
                && !seen[ny as usize * exwidth + nx as usize]
            {
                seen[ny as usize * exwidth + nx as usize] = true;
                queue.push_back((
                    if exgrid[ny as usize * exwidth + nx as usize] == b'O' {
                        steps + 1
                    } else {
                        steps
                    },
                    (nx, ny),
                ));
            }
        }
    }
}
