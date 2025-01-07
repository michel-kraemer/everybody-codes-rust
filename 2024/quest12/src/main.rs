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

/// Simulate catapult throw starting at (cx, cy) with the given strength and
/// check if the projectile is exactly at (optx, opty) after the given time
fn can_hit(mut cx: i32, mut cy: i32, optx: i32, opty: i32, strength: i32, time: i32) -> bool {
    let mut remaining_time = time;

    // positive diagonal
    let d1 = strength.min(remaining_time);
    cx += d1;
    cy += d1;
    remaining_time -= d1;
    if remaining_time == 0 && cx == optx && cy == opty {
        return true;
    }

    // horizontal
    let d2 = strength.min(remaining_time);
    cx += d2;
    remaining_time -= d2;
    if remaining_time == 0 && cx == optx && cy == opty {
        return true;
    }

    // negative diagonal
    let d3 = cy.min(remaining_time);
    cx += d3;
    cy -= d3;
    remaining_time -= d3;
    if remaining_time == 0 && cx == optx && cy == opty {
        return true;
    }

    false
}

fn main() {
    // part 1 and 2
    for part in [1, 2] {
        let targets = parse_file(&format!("everybody_codes_e2024_q12_p{}.txt", part));
        let mut total1 = 0;
        for tower in [0, 1, 2] {
            for &(tx, ty, hit_points) in &targets {
                for strength in 1..=tx {
                    if can_hit(0, tower, tx, ty, strength, tx) {
                        total1 += (tower + 1) * strength * hit_points;
                        break;
                    }
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

        // time until collision
        let time = optx;

        let mut min = i32::MAX;
        for tower in [0, 1, 2] {
            for strength in 1..=time {
                if can_hit(0, tower, optx, opty, strength, time) {
                    min = min.min((tower + 1) * strength);
                    break;
                }
            }
        }
        total3 += min;
    }
    println!("{}", total3);
}
