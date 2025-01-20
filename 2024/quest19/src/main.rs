use std::fs;

fn rotate<T>(grid: &mut [T], width: usize, x: usize, y: usize, left: bool)
where
    T: Copy,
{
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

fn decrypt<T>(grid: &mut [T], width: usize, height: usize, instructions: &[u8])
where
    T: Copy,
{
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
}

fn apply_permutation<T>(dest: &mut [T], src: &[T], permutation: &[usize])
where
    T: Copy,
{
    for i in 0..src.len() {
        dest[i] = src[permutation[i]];
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

        let n = match part {
            1 => 1,
            2 => 100,
            _ => 1048576000,
        };

        // decrypt once to get permutation
        let mut permutation = (0..grid.len()).collect::<Vec<_>>();
        decrypt(&mut permutation, width, height, instructions);

        // apply binary exponentiation to get to the answer really fast
        let mut k = n;
        while k > 0 {
            if k & 1 > 0 {
                // apply permutation to grid
                let mut ng = vec![b'.'; grid.len()];
                apply_permutation(&mut ng, &grid, &permutation);
                grid = ng;
            }

            // apply permutation to itself
            let mut np = vec![0; permutation.len()];
            apply_permutation(&mut np, &permutation, &permutation);
            permutation = np;

            k >>= 1;
        }

        // get the decrypted message
        let mut j = 0;
        while grid[j] != b'>' {
            j += 1;
        }
        j += 1;
        let mut total = 0;
        while grid[j].is_ascii_digit() {
            total *= 10;
            total += (grid[j] - b'0') as u64;
            j += 1;
        }
        println!("{}", total);
    }
}
