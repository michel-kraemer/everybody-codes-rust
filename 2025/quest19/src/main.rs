use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;
use std::ops::Range;

use rustc_hash::FxHashMap;

struct Opening {
    x: usize,
    y: usize,
    height: usize,
}

#[derive(PartialEq, Eq)]
struct State {
    flaps: usize,
    x: usize,
    y: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.flaps.cmp(&self.flaps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(path: &str) -> (Vec<Opening>, usize) {
    let input = fs::read_to_string(path).expect("Could not read file");
    let mut max_x = 0;
    let mut openings = Vec::new();
    for l in input.lines() {
        let mut parts = l.split(',');
        let o = Opening {
            x: parts.next().unwrap().parse::<usize>().unwrap(),
            y: parts.next().unwrap().parse::<usize>().unwrap(),
            height: parts.next().unwrap().parse::<usize>().unwrap(),
        };
        max_x = max_x.max(o.x);
        openings.push(o);
    }
    (openings, max_x)
}

fn next_openings(x: usize, openings: &[Opening]) -> Range<usize> {
    let left = openings.partition_point(|o| o.x <= x);
    let mut right = left;
    while right < openings.len() && openings[right].x == openings[left].x {
        right += 1;
    }
    left..right
}

fn shortest_path(openings: &[Opening], max_x: usize) -> usize {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        flaps: 0,
        x: 0,
        y: 0,
    });
    let mut seen: FxHashMap<(usize, usize), usize> = FxHashMap::default();

    while let Some(State { flaps, x, y }) = queue.pop() {
        if x == max_x {
            return flaps;
        }

        let nos = next_openings(x, openings);
        for no in nos {
            // every odd position is impossible to reach
            let mut no_min_y = openings[no].y;
            if !(no_min_y + openings[no].x).is_multiple_of(2) {
                no_min_y += 1;
            }
            let mut no_max_y = openings[no].y + openings[no].height - 1;
            if !(no_max_y + openings[no].x).is_multiple_of(2) {
                no_max_y -= 1;
            }

            // compute reachable minimum and maximum positions
            no_min_y = no_min_y.max(y.saturating_sub(openings[no].x - x));
            no_max_y = no_max_y.min(y + (openings[no].x - x));

            // take 2 steps - every odd position is impossible to reach
            for ny in (no_min_y..=no_max_y).step_by(2) {
                let mut nx = x;
                let mut new_flaps = flaps;
                if ny > y {
                    nx += ny - y;
                    new_flaps += ny - y;
                } else if ny < y {
                    nx += y - ny;
                }

                new_flaps += (openings[no].x - nx) / 2;
                nx = openings[no].x;

                let old = seen.get(&(nx, ny)).unwrap_or(&usize::MAX);
                if new_flaps < *old {
                    seen.insert((nx, ny), new_flaps);
                    queue.push(State {
                        flaps: new_flaps,
                        x: nx,
                        y: ny,
                    });
                }
            }
        }
    }

    unreachable!();
}

fn main() {
    for part in 1..=3 {
        let (openings, max_x) = parse(&format!("everybody_codes_e2025_q19_p{part}.txt"));
        println!("{}", shortest_path(&openings, max_x));
    }
}
