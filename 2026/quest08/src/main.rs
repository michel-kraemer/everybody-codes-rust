use std::{fs, ops::Range};

struct State {
    width: usize,
    height: usize,
    grid: Vec<i32>,
    grow: bool,
    round: u64,
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

        assert!(
            !width.is_multiple_of(2),
            "hexagonal grid must have an odd width"
        );
        assert!(
            !height.is_multiple_of(2),
            "hexagonal grid must have an odd height"
        );

        Self {
            width,
            height,
            grid,
            grow,
            round: 0,
            energy: 0,
            warriors: 0,
        }
    }

    fn upper_half(&self) -> Range<i32> {
        let end = self.height.div_ceil(2) as i32;
        let start = -(self.height as i32 / 2);
        start..end
    }

    fn lower_half(&self) -> Range<i32> {
        let start = self.height as i32 / 2;
        let end = self.height as i32 + self.height as i32 / 2;
        start..end
    }

    fn start_pos_left(&self, y: i32) -> (i32, i32) {
        let uh = self.upper_half();
        let x = if uh.contains(&y) {
            self.width as i32 / 2 - (y - uh.start)
        } else {
            let lh = self.lower_half();
            self.width as i32 / 2 - (lh.end - 1 - y)
        };
        if y < 0 {
            (x - y, 0)
        } else if (y as usize) >= self.height {
            (x + (y - (self.height as i32 - 1)), self.height as i32 - 1)
        } else {
            (x, y)
        }
    }

    fn start_pos_right(&self, y: i32) -> (i32, i32) {
        let (x, y) = self.start_pos_left(y);
        (self.width as i32 - x - 1, y)
    }

    fn compute(&mut self, mut x: i32, mut y: i32, dx: i32, dy: i32) {
        let mut curr = 0;

        let mut i = y as usize * self.width + x as usize;
        let inc = dy as isize * self.width as isize + dx as isize;

        while x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let d = self.grid[i];
            if d == 0 {
                break;
            }
            if d < 0 {
                if d == -1 {
                    self.grid[i] = 1;
                } else {
                    self.grid[i] = d + 1;
                }
            } else if d > curr {
                self.energy += 1;
                curr = d;
                if self.grow {
                    if d == 9 {
                        self.warriors += 1;
                        self.grid[i] = -3;
                    } else {
                        self.grid[i] = d + 1;
                    }
                }
            }

            x += dx;
            y += dy;
            i = i.wrapping_add_signed(inc);

            if curr > 0 {
                curr -= 1;
            }
        }
    }

    fn down_right(&mut self) {
        for y in self.upper_half() {
            let (x, y) = self.start_pos_left(y);
            self.compute(x, y, 1, 1);
        }
    }

    fn down_left(&mut self) {
        for y in self.upper_half() {
            let (x, y) = self.start_pos_right(y);
            self.compute(x, y, -1, 1);
        }
    }

    fn left(&mut self) {
        for y in 0..self.height {
            let (x, y) = self.start_pos_right(y as i32);
            self.compute(x, y, -2, 0);
        }
    }

    fn up_left(&mut self) {
        for y in self.lower_half() {
            let (x, y) = self.start_pos_right(y);
            self.compute(x, y, -1, -1);
        }
    }

    fn up_right(&mut self) {
        for y in self.lower_half() {
            let (x, y) = self.start_pos_left(y);
            self.compute(x, y, 1, -1);
        }
    }

    fn right(&mut self) {
        for y in 0..self.height {
            let (x, y) = self.start_pos_left(y as i32);
            self.compute(x, y, 2, 0);
        }
    }

    fn round(&mut self) {
        match self.round % 6 {
            0 => self.down_right(),
            1 => self.down_left(),
            2 => self.left(),
            3 => self.up_left(),
            4 => self.up_right(),
            5 => self.right(),
            _ => unreachable!(),
        };
        self.round += 1;
    }
}

fn main() {
    // part 1
    let mut state = State::parse("everybody_codes_e2026_q08_p1.txt", false);
    state.round();
    println!("{}", state.energy);

    // part 2
    let mut state = State::parse("everybody_codes_e2026_q08_p2.txt", false);
    for _ in 0..6 {
        state.round();
    }
    println!("{}", state.energy);

    // part 3
    let mut state = State::parse("everybody_codes_e2026_q08_p3.txt", true);
    loop {
        state.round();
        if state.warriors >= 100_000 {
            break;
        }
    }
    println!("{}", state.energy);
}
