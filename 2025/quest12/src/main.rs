use std::collections::VecDeque;
use std::fs;

fn parse(path: &str) -> (usize, usize, Vec<u8>) {
    let input = fs::read_to_string(path).expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();
    (width, height, grid)
}

fn count_seen(s: &[bool]) -> usize {
    s.iter().filter(|v| **v).count()
}

fn set_on_fire(grid: &[u8], width: usize, height: usize, start: (usize, usize)) -> Vec<bool> {
    let mut queue = VecDeque::new();
    let mut seen = vec![false; width * height];

    queue.push_back(start);
    seen[start.1 * width + start.0] = true;

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && (nx as usize) < width
                && ny >= 0
                && (ny as usize) < height
                && grid[ny as usize * width + nx as usize] <= grid[y * width + x]
                && !seen[ny as usize * width + nx as usize]
            {
                seen[ny as usize * width + nx as usize] = true;
                queue.push_back((nx as usize, ny as usize));
            }
        }
    }

    seen
}

fn merge(s: &mut [bool], other: &[bool]) {
    for (i, c) in s.iter_mut().enumerate() {
        if other[i] {
            *c = true;
        }
    }
}

fn main() {
    // part 1
    let (width, height, grid) = parse("everybody_codes_e2025_q12_p1.txt");
    println!("{}", count_seen(&set_on_fire(&grid, width, height, (0, 0))));

    // part 1
    let (width, height, grid) = parse("everybody_codes_e2025_q12_p2.txt");
    let mut s = set_on_fire(&grid, width, height, (0, 0));
    merge(
        &mut s,
        &set_on_fire(&grid, width, height, (width - 1, height - 1)),
    );
    println!("{}", count_seen(&s));

    // part 3
    let (width, height, grid) = parse("everybody_codes_e2025_q12_p3.txt");

    // find first barrel
    let mut max = 0;
    let mut max_seen = vec![false; width * height];
    for y in 0..height {
        for x in 0..width {
            let s = set_on_fire(&grid, width, height, (x, y));
            let sm = count_seen(&s);
            if sm > max {
                max = sm;
                max_seen = s;
            }
        }
    }

    // find second barrel
    let mut max2 = 0;
    let mut max_seen2 = vec![false; width * height];
    for y in 0..height {
        for x in 0..width {
            if !max_seen[y * width + x] {
                let mut s = set_on_fire(&grid, width, height, (x, y));
                merge(&mut s, &max_seen);
                let sm = count_seen(&s);
                if sm > max2 {
                    max2 = sm;
                    max_seen2 = s;
                }
            }
        }
    }

    // find third barrel
    let mut max3 = 0;
    for y in 0..height {
        for x in 0..width {
            if !max_seen2[y * width + x] {
                let mut s = set_on_fire(&grid, width, height, (x, y));
                merge(&mut s, &max_seen2);
                let sm = count_seen(&s);
                if sm > max3 {
                    max3 = sm;
                }
            }
        }
    }

    println!("{max3}");
}
