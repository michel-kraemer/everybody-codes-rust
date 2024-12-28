use std::fs;

pub const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub const CLOCKWISE: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn parse(filename: &str) -> (Vec<usize>, usize, usize) {
    let input = fs::read_to_string(filename).unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .into_iter()
        .flat_map(|l| {
            l.as_bytes()
                .iter()
                .map(|b| match b {
                    b'.' => 0,
                    b'#' => 1,
                    _ => panic!(),
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();
    (grid, width, height)
}

fn dig(mut grid: Vec<usize>, width: usize, height: usize, dirs: &[(i64, i64)]) -> usize {
    let mut level = 2;
    loop {
        let mut changed = false;
        let mut new_grid = grid.clone();

        for y in 0..height {
            for x in 0..width {
                let c = grid[y * width + x];
                if c == 0 {
                    continue;
                }

                let mut all_same = true;
                for (dx, dy) in dirs {
                    let nx = x as i64 + dx;
                    let ny = y as i64 + dy;
                    let nc = if nx >= 0 && ny >= 0 && nx < width as i64 && ny < height as i64 {
                        grid[ny as usize * width + nx as usize]
                    } else {
                        0
                    };

                    if nc != c {
                        all_same = false;
                        break;
                    }
                }

                if all_same {
                    new_grid[y * width + x] = level;
                    changed = true;
                }
            }
        }

        grid = new_grid;
        if !changed {
            break;
        }

        level += 1;
    }

    grid.iter().sum::<usize>()
}

fn main() {
    let (grid, width, height) = parse("everybody_codes_e2024_q03_p1.txt");
    println!("{}", dig(grid, width, height, &DIRS));

    let (grid, width, height) = parse("everybody_codes_e2024_q03_p2.txt");
    println!("{}", dig(grid, width, height, &DIRS));

    let (grid, width, height) = parse("everybody_codes_e2024_q03_p3.txt");
    println!("{}", dig(grid, width, height, &CLOCKWISE));
}
