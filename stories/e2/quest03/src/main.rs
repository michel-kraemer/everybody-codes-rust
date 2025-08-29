use std::fs;

use rustc_hash::FxHashSet;

#[cfg(feature = "visualize")]
use skia_safe::{Color, EncodedImageFormat, Paint, Path, surfaces};

#[derive(Clone, Debug)]
struct Die {
    faces: Vec<i32>,
    seed: usize,
    pulse: usize,
    front: usize,
    roll_number: usize,
}

impl Die {
    fn new(faces: Vec<i32>, seed: usize) -> Self {
        Self {
            faces,
            seed,
            pulse: seed,
            front: 0,
            roll_number: 0,
        }
    }

    fn next(&mut self) -> i32 {
        self.roll_number += 1;
        let spin = self.roll_number * self.pulse;
        self.front += spin;
        let score = self.faces[self.front % self.faces.len()];
        self.pulse = (self.pulse + spin) % self.seed;
        self.pulse = self.pulse + 1 + self.roll_number + self.seed;
        score
    }
}

fn parse_die(line: &str) -> Die {
    let parts = line.split(" ").collect::<Vec<_>>();
    let (_, faces) = parts[1].split_once("=").unwrap();
    let (_, seed) = parts[2].split_once("=").unwrap();

    let faces = faces[1..faces.len() - 1].split(",").collect::<Vec<_>>();
    let faces = faces
        .iter()
        .map(|f| f.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let seed = seed.parse::<usize>().unwrap();

    Die::new(faces, seed)
}

fn try_path(
    grid: &[Vec<i32>],
    x: usize,
    y: usize,
    mut die: Die,
    taken: &mut [Vec<bool>],
    seen: &mut FxHashSet<(usize, usize, usize)>,
) {
    let mut players = vec![(x, y)];
    while !players.is_empty() {
        let score = die.next();
        let mut new_players = Vec::new();
        for (px, py) in players {
            for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = px as i32 + dx;
                let ny = py as i32 + dy;
                if nx >= 0
                    && (nx as usize) < grid[0].len()
                    && ny >= 0
                    && (ny as usize) < grid.len()
                    && grid[ny as usize][nx as usize] == score
                    && seen.insert((die.roll_number, nx as usize, ny as usize))
                {
                    taken[ny as usize][nx as usize] = true;
                    new_players.push((nx as usize, ny as usize));
                }
            }
        }
        players = new_players;
    }
}

fn main() {
    // part 1
    let input = fs::read_to_string("everybody_codes_e2_q03_p1.txt").expect("Could not read file");
    let mut dice = input.lines().map(parse_die).collect::<Vec<_>>();
    let mut sum = 0;
    while sum < 10000 {
        sum += dice.iter_mut().map(|d| d.next()).sum::<i32>();
    }
    println!("{}", dice[0].roll_number);

    // part 2
    let input = fs::read_to_string("everybody_codes_e2_q03_p2.txt").expect("Could not read file");
    let (dice, track) = input.split_once("\n\n").unwrap();
    let mut dice = dice.lines().map(parse_die).collect::<Vec<_>>();
    let track = track
        .as_bytes()
        .iter()
        .map(|b| (b - b'0') as i32)
        .collect::<Vec<_>>();
    let mut players = vec![0; dice.len()];
    let mut won = Vec::new();
    while won.len() < players.len() {
        for (i, die) in dice.iter_mut().enumerate() {
            let score = die.next();
            if players[i] < track.len() && score == track[players[i]] {
                players[i] += 1;
                if players[i] == track.len() {
                    won.push(i + 1);
                }
            }
        }
    }
    println!(
        "{}",
        won.into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    // part 3
    let input = fs::read_to_string("everybody_codes_e2_q03_p3.txt").expect("Could not read file");
    let (dice, grid) = input.split_once("\n\n").unwrap();
    let dice = dice.lines().map(parse_die).collect::<Vec<_>>();
    let grid = grid
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|b| (b - b'0') as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut taken = vec![vec![false; grid[0].len()]; grid.len()];
    for die in dice {
        let mut seen = FxHashSet::default();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                try_path(&grid, x, y, die.clone(), &mut taken, &mut seen);
            }
        }
    }
    let mut num_taken = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if taken[y][x] {
                num_taken += 1;
            }
        }
    }
    println!("{num_taken}");

    #[cfg(feature = "visualize")]
    {
        let mut surface =
            surfaces::raster_n32_premul((grid[0].len() as i32, grid.len() as i32)).unwrap();
        let mut paint = Paint::default();
        paint.set_color(Color::BLACK);
        paint.set_anti_alias(false);
        paint.set_stroke_width(1.0);
        surface.canvas().clear(Color::WHITE);
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if taken[y][x] {
                    surface.canvas().draw_point((x as i32, y as i32), &paint);
                }
            }
        }
        let image = surface.image_snapshot();
        let mut context = surface.direct_context();
        let data = image
            .encode(context.as_mut(), EncodedImageFormat::PNG, None)
            .unwrap();
        fs::write("everybody_codes_e2_q03_p3.png", data.as_bytes()).unwrap();
    }
}
