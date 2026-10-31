use crate::grid::{CLOCKWISE, Grid};

mod grid;

const LAND: u8 = b'#';
const WATER: u8 = b'.';
const SAND: u8 = b'=';
const TREE: u8 = b'@';

/// Count how many cells in the grid equal the given byte
fn count_total(grid: &Grid, c: u8) -> usize {
    grid.grid.iter().filter(|b| **b == c).count()
}

/// Add neighbors of a tree to a queue if they are not land
fn add_tree_neighbors_to_queue(grid: &Grid, x: i32, y: i32, queue: &mut Vec<(i32, i32)>) {
    for (dx, dy) in CLOCKWISE {
        let nx = x + dx;
        let ny = y + dy;
        if grid.has(nx, ny) && grid.get(nx, ny) != LAND {
            queue.push((nx, ny));
        }
    }
}

/// Grow land cells from the center of the grid
fn mark_land(grid: &mut Grid) {
    assert!(grid.width() % 2 != 0, "number of columns must be odd");
    assert!(grid.height() % 2 != 0, "number of rows must be odd");

    let mut queue = Vec::new();

    // plant first tree and add its neighbors to queue
    grid.set(grid.width() / 2, grid.height() / 2, LAND);
    add_tree_neighbors_to_queue(grid, grid.width() / 2, grid.height() / 2, &mut queue);

    while let Some((x, y)) = queue.pop() {
        // count how many neighbors are LAND or '1'
        let mut count = 0;
        for (dx, dy) in CLOCKWISE {
            let nx = x + dx;
            let ny = y + dy;
            if grid.has(nx, ny) && (grid.get(nx, ny) == LAND || grid.get(nx, ny) == b'1') {
                count += 1;
            }
        }

        if count >= 5 {
            // plant new tree and add its neighbors to queue
            grid.set(x, y, LAND);
            add_tree_neighbors_to_queue(grid, x, y, &mut queue);
        }
    }
}

/// Mark everything that is not LAND as WATER
fn mark_water(grid: &mut Grid) {
    for b in grid.grid.iter_mut() {
        if *b != LAND {
            *b = WATER;
        }
    }
}

/// Mark all sand cells
fn mark_sand(grid: &mut Grid) {
    // first pass: mark all LAND cells as sand if they are next to water
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(x, y) != LAND {
                continue;
            }
            for (dx, dy) in CLOCKWISE {
                let nx = x + dx;
                let ny = y + dy;
                if !grid.has(nx, ny) || grid.get(nx, ny) == WATER {
                    grid.set(x, y, SAND);
                    break;
                }
            }
        }
    }

    // Second pass: mark all LAND cells as sand if they have at least 5 other
    // neighboring sand cells. Repeat until the grid doesn't change anymore.
    loop {
        let mut changed = false;

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if grid.get(x, y) != LAND {
                    continue;
                }

                // count neighboring SAND cells
                let mut count = 0;
                for (dx, dy) in CLOCKWISE {
                    let nx = x + dx;
                    let ny = y + dy;
                    if grid.has(nx, ny) && grid.get(nx, ny) == SAND {
                        count += 1;
                    }
                }

                if count >= 5 {
                    grid.set(x, y, SAND);
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }
}

/// Mark all tree cells
fn mark_trees(grid: &mut Grid) {
    // first pass: mark tree cells according to the pattern in the problem
    // description
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(x, y) != LAND {
                continue;
            }

            let mut ok = true;
            'outer: for nx in x - 2..=x + 2 {
                for ny in y - 2..=y + 2 {
                    if !grid.has(nx, ny) {
                        ok = false;
                        break 'outer;
                    }

                    if (nx == x - 2 && (ny == y - 2 || ny == y + 2))
                        || (nx == x + 2 && (ny == y - 2 || ny == y + 2))
                        || (nx == x && ny == y)
                    {
                        // skip corner cells and center cell
                        continue;
                    }

                    if grid.get(nx, ny) != LAND && grid.get(nx, ny) != TREE {
                        ok = false;
                        break 'outer;
                    }
                }
            }

            if ok {
                grid.set(x, y, TREE);
            }
        }
    }

    // second and third passes: grow tree area
    for _ in 0..2 {
        let mut new_trees = Vec::new();
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if grid.get(x, y) != LAND {
                    continue;
                }

                // count neighboring tree and sand cells
                let mut count_trees = 0;
                let mut count_sand = 0;
                for (dx, dy) in CLOCKWISE {
                    let nx = x + dx;
                    let ny = y + dy;
                    if grid.has(nx, ny) && grid.get(nx, ny) == TREE {
                        count_trees += 1;
                    }
                    if grid.has(nx, ny) && grid.get(nx, ny) == SAND {
                        count_sand += 1;
                        break;
                    }
                }

                if count_trees >= 4 && count_sand == 0 {
                    // push new tree positions into a queue so we don't count
                    // them in the current pass
                    new_trees.push((x, y));
                }
            }
        }

        for (x, y) in new_trees {
            grid.set(x, y, TREE);
        }
    }
}

#[cfg(feature = "visualize")]
fn visualize(grid: &Grid, output_path: &str) {
    use rand::RngExt as _;
    use skia_safe::{Color, EncodedImageFormat, Paint, surfaces};

    let mut rng = rand::rng();
    let mut surface = surfaces::raster_n32_premul((grid.width(), grid.height())).expect("surface");

    const COLOR_LAND: Color = Color::from_rgb(6, 206, 6);
    const COLOR_WATER: Color = Color::from_rgb(27, 141, 247);
    const COLOR_SAND: Color = Color::from_rgb(247, 210, 4);
    const COLOR_TREE: Color = Color::from_rgb(3, 94, 3);

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let mut hsv = match grid.get(x, y) {
                LAND => COLOR_LAND.to_hsv(),
                WATER => COLOR_WATER.to_hsv(),
                SAND => COLOR_SAND.to_hsv(),
                TREE => COLOR_TREE.to_hsv(),
                _ => unreachable!(),
            };
            hsv.v += rng.random_range(-0.02..=0.02);

            let mut paint = Paint::default();
            paint.set_color(hsv.to_color(255));

            surface.canvas().draw_point((x, y), &paint);
        }
    }

    let image = surface.image_snapshot();
    let mut context = surface.direct_context();
    let data = image
        .encode(context.as_mut(), EncodedImageFormat::PNG, None)
        .unwrap();
    std::fs::write(output_path, data).expect("Could not write image file");
}

fn main() {
    // part 1
    let mut grid =
        Grid::from_path("everybody_codes_e2026_q05_p1.txt").expect("Could not read file");
    mark_land(&mut grid);
    mark_water(&mut grid);
    println!("{}", count_total(&grid, LAND));

    #[cfg(feature = "visualize")]
    visualize(&grid, "everybody_codes_e2026_q05_p1.png");

    // part 2
    let mut grid =
        Grid::from_path("everybody_codes_e2026_q05_p2.txt").expect("Could not read file");
    mark_land(&mut grid);
    mark_water(&mut grid);
    mark_sand(&mut grid);
    println!("{}", count_total(&grid, SAND));

    #[cfg(feature = "visualize")]
    visualize(&grid, "everybody_codes_e2026_q05_p2.png");

    // part 3
    let mut grid =
        Grid::from_path("everybody_codes_e2026_q05_p3.txt").expect("Could not read file");
    mark_land(&mut grid);
    mark_water(&mut grid);
    mark_sand(&mut grid);
    mark_trees(&mut grid);
    println!("{}", count_total(&grid, TREE));

    #[cfg(feature = "visualize")]
    visualize(&grid, "everybody_codes_e2026_q05_p3.png");
}
