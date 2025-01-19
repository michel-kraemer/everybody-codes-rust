use std::fs;

fn rotate(grid: &mut [u8], width: usize, x: usize, y: usize, left: bool) {
    let dx = if left { -1 } else { 1 };
    let mut nx = (x as i32) + dx;
    let mut ny = y - 1;
    let sc = grid[ny * width + nx as usize];
    for _ in 0..2 {
        grid[ny * width + nx as usize] = grid[ny * width + (nx - dx) as usize];
        nx -= dx;
    }
    for _ in 0..2 {
        grid[ny * width + nx as usize] = grid[(ny + 1) * width + nx as usize];
        ny += 1;
    }
    for _ in 0..2 {
        grid[ny * width + nx as usize] = grid[ny * width + (nx + dx) as usize];
        nx += dx;
    }
    for _ in 0..1 {
        grid[ny * width + nx as usize] = grid[(ny - 1) * width + nx as usize];
        ny -= 1;
    }
    grid[ny * width + nx as usize] = sc;
}

fn decrypt(grid: &mut [u8], width: usize, height: usize, instructions: &[u8]) -> u64 {
    loop {
        let mut i = 0;
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if instructions[i % instructions.len()] == b'L' {
                    rotate(grid, width, x, y, true);
                } else {
                    rotate(grid, width, x, y, false);
                }
                i += 1;
            }
        }

        // the decrypted message is always 16 digits long
        for j in 0..grid.len() {
            if grid[j] == b'>' {
                let mut m = j + 1;
                let mut total = 0;
                let mut len = 0;
                while m < grid.len() && grid[m].is_ascii_digit() {
                    total *= 10;
                    total += (grid[m] - b'0') as u64;
                    m += 1;
                    len += 1;
                }
                if len == 16 && m < grid.len() && grid[m] == b'<' {
                    return total;
                } else {
                    break;
                }
            }
        }
    }
}

fn main() {
    for part in [1, 2, 3] {
        let input = fs::read_to_string(format!("everybody_codes_e2024_q19_p{}.txt", part)).unwrap();

        let (instructions, grid) = input.split_once("\n\n").unwrap();
        let instructions = instructions.as_bytes();

        let lines = grid.lines().collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();
        let mut grid = lines
            .into_iter()
            .flat_map(|l| l.as_bytes().iter().copied())
            .collect::<Vec<_>>();

        println!("{}", decrypt(&mut grid, width, height, instructions));
    }
}
