use std::collections::HashMap;
use std::fs;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    CW,
    CCW,
}

struct Hand {
    ticks: i64,
    direction: Direction,
    current: i64,
    remaining_moves: i64,
}

fn parse(input: &str) -> Vec<Hand> {
    let lines = input.lines().collect::<Vec<_>>();

    let ticks = lines[0][6..]
        .split(':')
        .map(|h| h.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let directions = lines[1][10..]
        .split(':')
        .map(|d| {
            if d == "CW" {
                Direction::CW
            } else {
                Direction::CCW
            }
        })
        .collect::<Vec<_>>();
    let current_positions = lines[2][8..]
        .split(':')
        .map(|h| h.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let moves = lines[3][5..]
        .split(':')
        .map(|h| h.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut hands = Vec::new();
    for i in 0..ticks.len() {
        hands.push(Hand {
            ticks: ticks[i],
            direction: directions[i],
            current: current_positions[i],
            remaining_moves: moves[i],
        });
    }

    hands
}

fn calibrate(input: &str) -> i64 {
    let mut hands = parse(input);

    // how many ticks does the whole clock have
    let all_ticks = hands.iter().map(|c| c.ticks).product::<i64>();

    // map current angles to a tuple of seconds and hits
    let mut seen: HashMap<Vec<i64>, (i64, i64)> = HashMap::new();

    let mut hits = 0;
    let mut seconds = 0;
    while hands.iter().any(|c| c.remaining_moves > 0) {
        // compute current angles and next angles
        let mut current_angles = Vec::with_capacity(hands.len());
        let mut next_angles = Vec::with_capacity(hands.len());

        for h in &hands {
            let angle_inc = all_ticks / h.ticks;

            let current_angle = if h.direction == Direction::CW {
                (h.current * angle_inc) % all_ticks
            } else {
                let mut ca = (h.current * angle_inc).rem_euclid(all_ticks);
                if ca == 0 {
                    ca = all_ticks;
                }
                ca
            };

            let next_angle = if h.remaining_moves > 0 {
                if h.direction == Direction::CW {
                    let mut na = ((h.current + 1) * angle_inc) % all_ticks;
                    if na == 0 {
                        na = all_ticks;
                    }
                    na
                } else {
                    ((h.current - 1) * angle_inc).rem_euclid(all_ticks)
                }
            } else {
                current_angle
            };

            current_angles.push(current_angle);
            next_angles.push(next_angle);
        }

        // check if we've seen this situation before
        if let Some(prev) = seen.get(&current_angles) {
            // We found a cycle. Skip ahead in time.
            let cycle_hits = hits - prev.1;
            let cycle_seconds = seconds - prev.0;

            // only consider hands that are still moving
            let mut hands_to_process = hands
                .iter_mut()
                .filter(|c| c.remaining_moves > 0)
                .collect::<Vec<_>>();

            // compute how many cycles we can skip ahead without completely
            // using all moves of a hand
            let min = hands_to_process
                .iter()
                .map(|c| c.remaining_moves)
                .min()
                .unwrap();
            let cycles = (min - 1) / cycle_seconds;

            // skip ahead
            for h in &mut hands_to_process {
                h.remaining_moves -= cycle_seconds * cycles;
                if h.direction == Direction::CW {
                    h.current += cycle_seconds * cycles;
                } else {
                    h.current -= cycle_seconds * cycles;
                }
            }
            hits += cycle_hits * cycles;

            // clear cache
            seen.clear();
        } else {
            seen.insert(current_angles.clone(), (seconds, hits));
        }

        // look for pair-wise collisions
        for a in 0..hands.len() {
            for b in a + 1..hands.len() {
                let curr_a = current_angles[a];
                let curr_b = current_angles[b];
                let next_a = next_angles[a];
                let next_b = next_angles[b];

                if hands[a].direction == hands[b].direction {
                    if (curr_a < curr_b && next_a > next_b) || (curr_a > curr_b && next_a < next_b)
                    {
                        // one hand has passed the other one
                        hits += 1;
                    }
                } else if hands[a].direction == Direction::CW
                    && hands[b].direction == Direction::CCW
                {
                    if curr_a < curr_b && next_a > next_b {
                        hits += 1;
                    }
                } else {
                    if curr_a > curr_b && next_a < next_b {
                        hits += 1;
                    }
                }

                // check if the both hands were at different positions before
                // but now land at the same one
                if next_a % all_ticks == next_b % all_ticks
                    && curr_a % all_ticks != curr_b % all_ticks
                {
                    hits += 1;
                }
            }
        }

        // move hands
        for h in &mut hands {
            if h.remaining_moves > 0 {
                if h.direction == Direction::CW {
                    h.current += 1;
                } else {
                    h.current -= 1;
                }
                h.remaining_moves -= 1;

                if h.remaining_moves == 0 {
                    // clear cache as soon as one hand has stopped, because this
                    // invalidates all previous states
                    seen.clear();
                }
            }
        }

        seconds += 1;
    }

    hits
}

fn simulate(path: &str) -> i64 {
    let input = fs::read_to_string(path).expect("Could not read file");
    let blocks = input.trim().split("\n\n").collect::<Vec<_>>();
    blocks.into_iter().map(calibrate).sum()
}

fn main() {
    println!("{}", simulate("everybody_codes_e2026_q13_p1.txt"));
    println!("{}", simulate("everybody_codes_e2026_q13_p2.txt"));
    println!("{}", simulate("everybody_codes_e2026_q13_p3.txt"));
}
