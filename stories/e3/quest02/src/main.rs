use grid::*;

mod grid;

fn fill(grid: &mut Grid<char>, start_x: i64, start_y: i64, c: char) {
    if grid.get(start_x, start_y) != '.' {
        return;
    }
    let mut queue = vec![(start_x, start_y)];
    while let Some((x, y)) = queue.pop() {
        grid.set(x, y, c);
        for (dx, dy) in DIRS {
            let nx = x + dx;
            let ny = y + dy;
            if grid.has(nx, ny) && grid.get(nx, ny) == '.' {
                queue.push((nx, ny));
            }
        }
    }
}

fn all_bones_surrounded(grid: &Grid<char>) -> bool {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(x, y) == '#' {
                for (dx, dy) in DIRS {
                    let nx = x + dx;
                    let ny = y + dy;
                    if !grid.has(nx, ny) || grid.get(nx, ny) == '.' {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn run<P>(mut grid: Grid<char>, instructions: &[(i64, i64)], predicate: P) -> u64
where
    P: Fn(&Grid<char>) -> bool,
{
    let mut instructions = instructions.iter().cycle();

    let mut pos = (0, 0);
    for (x, y, c) in grid.iter() {
        if c == '@' {
            pos = (x, y);
            break;
        }
    }

    let mut steps = 0;
    while !predicate(&grid) {
        let ins = instructions.next().unwrap();
        let mut nx = pos.0 + ins.0;
        let mut ny = pos.1 + ins.1;

        // extend grid if necessary
        if !grid.has(nx, ny) {
            if nx < 0 {
                let mut new_grid = Grid {
                    grid: vec![vec!['.'; grid.width() as usize + 1]; grid.height() as usize],
                };
                for y in 0..grid.height() {
                    for x in 0..grid.width() {
                        new_grid.set(x + 1, y, grid.get(x, y));
                    }
                }
                grid = new_grid;
                nx += 1;
                pos.0 += 1;
            } else if ny < 0 {
                let mut new_grid = Grid {
                    grid: vec![vec!['.'; grid.width() as usize]; grid.height() as usize + 1],
                };
                for y in 0..grid.height() {
                    for x in 0..grid.width() {
                        new_grid.set(x, y + 1, grid.get(x, y));
                    }
                }
                grid = new_grid;
                ny += 1;
                pos.1 += 1;
            } else if nx == grid.width() {
                let mut new_grid = Grid {
                    grid: vec![vec!['.'; grid.width() as usize + 1]; grid.height() as usize],
                };
                for y in 0..grid.height() {
                    for x in 0..grid.width() {
                        new_grid.set(x, y, grid.get(x, y));
                    }
                }
                grid = new_grid;
            } else {
                let mut new_grid = Grid {
                    grid: vec![vec!['.'; grid.width() as usize]; grid.height() as usize + 1],
                };
                for y in 0..grid.height() {
                    for x in 0..grid.width() {
                        new_grid.set(x, y, grid.get(x, y));
                    }
                }
                grid = new_grid;
            }
        }

        if grid.get(nx, ny) != '.' {
            // instruction does not work
            continue;
        }

        // take step
        steps += 1;
        grid.set(nx, ny, '+');
        pos = (nx, ny);

        // flood-fill grid from all border cells
        let width = grid.width();
        let height = grid.height();
        for x in 0..width {
            fill(&mut grid, x, 0, 'F');
            fill(&mut grid, x, height - 1, 'F');
        }
        for y in 0..height {
            fill(&mut grid, 0, y, 'F');
            fill(&mut grid, width - 1, y, 'F');
        }

        // clear all filled cells again and instead fill those that could not be
        // reached from any border cell
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let c = grid.get(x, y);
                if c == '.' {
                    grid.set(x, y, '+');
                } else if c == 'F' {
                    grid.set(x, y, '.');
                }
            }
        }
    }

    steps
}

fn main() {
    // part 1
    let mut grid =
        grid::read_to_grid("everybody_codes_e3_q02_p1.txt").expect("Could not read file");
    let instructions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dest = (0, 0);
    for (x, y, c) in grid.iter() {
        if c == '#' {
            dest = (x, y);
            grid.set(x, y, '.');
            break;
        }
    }
    println!(
        "{}",
        run(grid, &instructions, |g| g.get(dest.0, dest.1) == '+')
    );

    // part 2
    let grid = grid::read_to_grid("everybody_codes_e3_q02_p2.txt").expect("Could not read file");
    println!("{}", run(grid, &instructions, all_bones_surrounded));

    // part 3
    let grid = grid::read_to_grid("everybody_codes_e3_q02_p3.txt").expect("Could not read file");
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
    println!("{}", run(grid, &instructions, all_bones_surrounded));
}
