use std::fs;

fn parse_file(filename: &str) -> Vec<(i32, i32, i32)> {
    let input = fs::read_to_string(filename).unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let mut result = Vec::new();
    for (y, l) in lines.into_iter().rev().skip(1).enumerate() {
        for (x, c) in l.chars().skip(1).enumerate() {
            if c == 'H' {
                result.push((x as i32, y as i32, 2));
            } else if c == 'T' {
                result.push((x as i32, y as i32, 1));
            }
        }
    }
    result
}

/// Extrapolate height of projectile fired from height `cy` with the given
/// `strength` after the given `time`
fn extrapolate(mut cy: i32, strength: i32, time: i32) -> i32 {
    let mut remaining_time = time;

    // positive diagonal
    let d1 = strength.min(remaining_time);
    cy += d1;
    remaining_time -= d1;
    if remaining_time == 0 {
        return cy;
    }

    // horizontal
    let d2 = strength.min(remaining_time);
    remaining_time -= d2;
    if remaining_time == 0 {
        return cy;
    }

    // negative diagonal
    cy - remaining_time
}

/// Check if the tower at (cx, cy) can hit the target at (tx, ty), and if so,
/// return the required strength
fn can_hit(cx: i32, cy: i32, tx: i32, ty: i32) -> Option<i32> {
    // time until collision
    let time = tx - cx;

    let max_height = cy + time;
    if max_height == ty {
        // tower can hit the target in a straight line
        return Some(time);
    }

    if max_height < ty {
        // tower can never fire high enough to git the target
        return None;
    }

    // use binary search to find the required strength
    let mut min = 1;
    let mut max = time;
    while min < max {
        let strength = (min + max) / 2;
        let hy = extrapolate(cy, strength, time);
        if hy == ty {
            // hit!
            return Some(strength);
        }
        if hy > ty {
            max = strength;
        } else {
            min = strength + 1;
        }
    }

    // tower has always missed the target
    None
}

fn main() {
    // part 1 and 2
    for part in [1, 2] {
        let targets = parse_file(&format!("everybody_codes_e2024_q12_p{}.txt", part));
        let mut total1 = 0;
        for tower in [0, 1, 2] {
            for &(tx, ty, hit_points) in &targets {
                if let Some(strength) = can_hit(0, tower, tx, ty) {
                    total1 += (tower + 1) * strength * hit_points;
                }
            }
        }
        println!("{}", total1);
    }

    // part 3
    let input = fs::read_to_string("everybody_codes_e2024_q12_p3.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    let mut total3 = 0;
    for l in lines {
        let (mx, my) = l.split_once(" ").unwrap();
        let mx = mx.parse::<i32>().unwrap();
        let my = my.parse::<i32>().unwrap();

        // calculate optimal point of collision
        let optx = mx / 2;
        let opty = my - (mx - optx);

        let mut min = i32::MAX;
        for tower in [0, 1, 2] {
            if let Some(strength) = can_hit(0, tower, optx, opty) {
                min = min.min((tower + 1) * strength);
                break;
            }
        }
        total3 += min;
    }
    println!("{}", total3);
}
