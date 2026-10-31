use std::{
    fmt::{Display, Formatter},
    fs,
};

pub const CLOCKWISE: [(i32, i32); 8] = [
    (1, 0),   // →
    (1, 1),   // ↘︎
    (0, 1),   // ↓
    (-1, 1),  // ↙︎
    (-1, 0),  // ←
    (-1, -1), // ↖︎
    (0, -1),  // ↑
    (1, -1),  // ↗︎
];

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Grid {
    width: usize,
    height: usize,
    pub grid: Vec<u8>,
}

impl Grid {
    pub fn from_path(path: &str) -> Result<Self, std::io::Error> {
        let input = fs::read_to_string(path)?;
        let lines = input.lines().collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();
        let grid = lines
            .into_iter()
            .flat_map(|l| l.as_bytes())
            .copied()
            .collect::<Vec<_>>();
        Ok(Grid {
            width,
            height,
            grid,
        })
    }

    pub fn has(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width() && y >= 0 && y < self.height()
    }

    pub fn width(&self) -> i32 {
        self.width as i32
    }

    pub fn height(&self) -> i32 {
        self.height as i32
    }

    pub fn get(&self, x: i32, y: i32) -> u8 {
        self.grid[y as usize * self.width + x as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, c: u8) {
        self.grid[y as usize * self.width + x as usize] = c;
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        for (i, b) in self.grid.iter().enumerate() {
            if i > 0 && i % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", *b as char)?;
        }
        Ok(())
    }
}
