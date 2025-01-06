use std::fs;

fn parse_file(filename: &str) -> (Vec<u8>, usize, usize) {
    let input = fs::read_to_string(filename).unwrap();
    parse(&input)
}

fn parse(input: &str) -> (Vec<u8>, usize, usize) {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .into_iter()
        .flat_map(|l| l.as_bytes().iter().copied())
        .collect::<Vec<_>>();
    (grid, width, height)
}

fn try_solve(grid: &mut [u8], width: usize, sx: usize, sy: usize) -> Option<(usize, Vec<u8>)> {
    let mut used = vec![vec![false; 8]; 8];

    for y in 2..6 {
        'col: for x in 2..6 {
            for y1 in 0..8 {
                if (2..6).contains(&y1) {
                    continue;
                }
                let cy = grid[(sy + y1) * width + sx + x];
                for x1 in 0..8 {
                    if (2..6).contains(&x1) {
                        continue;
                    }
                    let cx = grid[(sy + y) * width + sx + x1];
                    if cx == cy {
                        used[y][x1] = true;
                        used[y1][x] = true;
                        grid[(sy + y) * width + sx + x] = cx;
                        continue 'col;
                    }
                }
            }
        }
    }

    for y in 2..6 {
        for x in 2..6 {
            if grid[(sy + y) * width + sx + x] != b'.' {
                continue;
            }
            let mut unused = Vec::new();
            for y1 in 0..8 {
                if (2..6).contains(&y1) {
                    continue;
                }
                let cy = grid[(sy + y1) * width + sx + x];
                if used[y1][x] {
                    continue;
                }
                for x1 in 0..8 {
                    if (2..6).contains(&x1) {
                        continue;
                    }
                    if used[y][x1] {
                        continue;
                    }
                    let cx = grid[(sy + y) * width + sx + x1];
                    if cx == b'?' && cy != b'?' {
                        unused.push((cy, x1, y));
                    } else if cx != b'?' && cy == b'?' {
                        unused.push((cx, x, y1));
                    }
                }
            }
            if unused.len() == 1 {
                let (c, x1, y1) = unused[0];
                grid[(sy + y) * width + sx + x] = c;
                grid[(sy + y1) * width + sx + x1] = c;
            }
        }
    }

    let mut solvable = true;
    let mut result = Vec::new();
    'row: for y in 2..6 {
        for x in 2..6 {
            let c = grid[(sy + y) * width + sx + x];
            if c == b'.' {
                solvable = false;
                break 'row;
            }
            result.push(c);
        }
    }

    if solvable {
        let mut power = 0;
        for (i, c) in result.iter().enumerate() {
            power += (i + 1) * (c - b'A' + 1) as usize;
        }
        Some((power, result))
    } else {
        None
    }
}

fn main() {
    // part 1
    let (mut grid1, width1, _) = parse_file("everybody_codes_e2024_q10_p1.txt");
    println!(
        "{}",
        try_solve(&mut grid1, width1, 0, 0)
            .unwrap()
            .1
            .iter()
            .map(|c| *c as char)
            .collect::<String>()
    );

    // part 2
    let input2 = fs::read_to_string("everybody_codes_e2024_q10_p2.txt").unwrap();
    let mut total = 0;
    for block in input2.split("\n\n") {
        let (mut grid2, width2, _) = parse(block);
        let w = (width2 + 1) / 9;

        for c in 0..w {
            if let Some((power, _)) = try_solve(&mut grid2, width2, c * 9, 0) {
                total += power;
            }
        }
    }
    println!("{}", total);

    // part 3
    let (mut grid3, width3, height3) = parse_file("everybody_codes_e2024_q10_p3.txt");

    let w = width3 / 6;
    let h = height3 / 6;

    let mut max_total = 0;
    loop {
        let mut total = 0;
        for r in 0..h {
            for c in 0..w {
                if let Some((power, _)) = try_solve(&mut grid3, width3, c * 6, r * 6) {
                    total += power;
                }
            }
        }

        if total > max_total {
            max_total = total;
        } else {
            break;
        }
    }
    println!("{}", max_total);
}
