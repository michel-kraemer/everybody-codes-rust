use std::fs;

fn parse(file: &str) -> Vec<usize> {
    let input = fs::read_to_string(file).expect("Could not read file");
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn count_intersections(new_thread: (usize, usize), threads: &[(usize, usize)]) -> usize {
    let r = new_thread.0 + 1..new_thread.1;
    let mut result = 0;
    for &old_thread in threads {
        if old_thread == new_thread || {
            (old_thread.0 < new_thread.0 && r.contains(&old_thread.1))
                || (r.contains(&old_thread.0) && old_thread.1 > new_thread.1)
        } {
            result += 1;
        }
    }
    result
}

fn add_range(grid: &mut [i64], x1: usize, y1: usize, x2: usize, y2: usize, size: usize) {
    if x1 >= size || y1 >= size {
        return;
    }
    grid[y1 * size + x1] += 1;
    if x2 < size {
        grid[y1 * size + x2] -= 1;
    }
    if y2 < size {
        grid[y2 * size + x1] -= 1;
    }
    if x2 < size && y2 < size {
        grid[y2 * size + x2] += 1;
    }
}

fn main() {
    // part 1
    let input = parse("everybody_codes_e2025_q08_p1.txt");
    let mut total = 0;
    for w in input.windows(2) {
        if w[0].abs_diff(w[1]) == 16 {
            total += 1;
        }
    }
    println!("{total}");

    // part 2
    let input = parse("everybody_codes_e2025_q08_p2.txt");
    let mut threads: Vec<(usize, usize)> = Vec::new();
    let mut total = 0;
    for w in input.windows(2) {
        let new_thread = if w[0] < w[1] {
            (w[0], w[1])
        } else {
            (w[1], w[0])
        };
        total += count_intersections(new_thread, &threads);
        threads.push(new_thread);
    }
    println!("{total}");

    // part 3
    const SIZE: usize = 256;
    let input = parse("everybody_codes_e2025_q08_p3.txt");
    let mut grid = vec![0i64; SIZE * SIZE];
    for w in input.windows(2) {
        let t = if w[0] < w[1] {
            (w[0] - 1, w[1] - 1)
        } else {
            (w[1] - 1, w[0] - 1)
        };
        let y1 = t.0 + 1;
        let x1 = t.1 + 1;
        let y2 = t.1;
        let x2 = t.0;
        add_range(&mut grid, x1, y1, SIZE + 1, y2, SIZE);
        add_range(&mut grid, y1, 0, y2, x2, SIZE);
        add_range(&mut grid, t.1, t.0, t.1 + 1, t.0 + 1, SIZE);
    }

    let mut max = 0;
    for y in 0..SIZE {
        let mut sum = 0;
        for x in 0..SIZE {
            sum += grid[y * SIZE + x];
            grid[y * SIZE + x] = sum;
        }
    }
    for x in 0..SIZE {
        let mut sum = 0;
        for y in 0..SIZE {
            sum += grid[y * SIZE + x];
            max = max.max(sum);
        }
    }

    println!("{max}");
}
