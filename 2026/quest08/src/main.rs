use std::fs;

struct State {
    width: usize,
    height: usize,
    grid: Vec<i32>,
    grow: bool,
    energy: u64,
    warriors: u64,
}

impl State {
    fn parse(path: &str, grow: bool) -> Self {
        let input = fs::read_to_string(path).expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();
        let grid = lines
            .iter()
            .flat_map(|l| {
                l.bytes()
                    .map(|b| if b == b'.' { 0 } else { (b - b'0') as i32 })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            width,
            height,
            grid,
            grow,
            energy: 0,
            warriors: 0,
        }
    }

    fn compute(&mut self, mut x: i32, mut y: i32, dx: i32, dy: i32) {
        let mut curr = 0;

        while x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            if self.grid[y as usize * self.width + x as usize] != 0 {
                let d = self.grid[y as usize * self.width + x as usize];
                if d > curr {
                    self.energy += 1;
                    curr = d;
                    if self.grow {
                        if d == 9 {
                            self.warriors += 1;
                            self.grid[y as usize * self.width + x as usize] = -4;
                        } else {
                            self.grid[y as usize * self.width + x as usize] = d + 1;
                        }
                    }
                }
            }

            x += dx;
            y += dy;

            if curr > 0 {
                curr -= 1;
            }
        }
    }

    fn down_right(&mut self) {
        for y in 0..self.height {
            self.compute(0, y as i32, 1, 1);
        }
        for x in 1..self.width {
            self.compute(x as i32, 0, 1, 1);
        }
    }

    fn down_left(&mut self) {
        for x in 0..self.width {
            self.compute(x as i32, 0, -1, 1);
        }
        for y in 1..self.height {
            self.compute(self.width as i32 - 1, y as i32, -1, 1);
        }
    }

    fn left(&mut self) {
        for y in 0..self.height {
            if y % 2 == self.height.div_ceil(2) % 2 {
                self.compute(self.width as i32 - 2, y as i32, -2, 0);
            } else {
                self.compute(self.width as i32 - 1, y as i32, -2, 0);
            }
        }
    }

    fn up_left(&mut self) {
        for x in 0..self.width {
            self.compute(x as i32, self.height as i32 - 1, -1, -1);
        }
        for y in 0..self.height - 1 {
            self.compute(self.width as i32 - 1, y as i32, -1, -1);
        }
    }

    fn up_right(&mut self) {
        for y in 0..self.height - 1 {
            self.compute(0, y as i32, 1, -1);
        }
        for x in 0..self.width {
            self.compute(x as i32, self.height as i32 - 1, 1, -1);
        }
    }

    fn right(&mut self) {
        for y in 0..self.height {
            if y % 2 == self.height.div_ceil(2) % 2 {
                self.compute(1, y as i32, 2, 0);
            } else {
                self.compute(0, y as i32, 2, 0);
            }
        }
    }

    fn inc(&mut self) {
        for i in 0..self.grid.len() {
            let d = self.grid[i];
            if d == -1 {
                self.grid[i] = 1;
            } else if d < 0 {
                self.grid[i] = d + 1;
            }
        }
    }
}

fn main() {
    // part 1
    let mut state = State::parse("everybody_codes_e2026_q08_p1.txt", false);
    state.down_right();
    println!("{}", state.energy);

    // part 2
    let mut state = State::parse("everybody_codes_e2026_q08_p2.txt", false);
    state.down_right();
    state.down_left();
    state.left();
    state.up_left();
    state.up_right();
    state.right();
    println!("{}", state.energy);

    // part 3
    let mut state = State::parse("everybody_codes_e2026_q08_p3.txt", true);
    const LIMIT: u64 = 100000;

    loop {
        state.down_right();
        state.inc();
        if state.warriors >= LIMIT {
            break;
        }

        state.down_left();
        state.inc();
        if state.warriors >= LIMIT {
            break;
        }

        state.left();
        state.inc();
        if state.warriors >= LIMIT {
            break;
        }

        state.up_left();
        state.inc();
        if state.warriors >= LIMIT {
            break;
        }

        state.up_right();
        state.inc();
        if state.warriors >= LIMIT {
            break;
        }

        state.right();
        state.inc();
        if state.warriors >= LIMIT {
            break;
        }
    }

    println!("{}", state.energy);
}
