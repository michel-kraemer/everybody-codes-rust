use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use z3::ast::Int;
use z3::{Optimize, SatResult};

#[derive(Debug, Clone)]
struct Cooridor {
    start_x: usize,
    end_x: usize,
    length: usize,
    sulphur_up: usize,
    sulphur_down: usize,
}

#[derive(Debug, Clone)]
struct Chamber {
    start_x: usize,
    end_x: usize,
    corridors: Vec<Cooridor>,
}

fn parse_grid(path: &str) -> (usize, usize, Vec<u8>) {
    let input = fs::read_to_string(path).expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .into_iter()
        .flat_map(|l| l.bytes())
        .collect::<Vec<_>>();
    (width, height, grid)
}

fn parse(path: &str) -> Vec<Chamber> {
    let (width, height, grid) = parse_grid(path);

    // find all chambers
    let mut chambers = Vec::new();
    let mut start_x = 0;
    let mut x = 1;
    while x < width {
        if grid[width + x] == b'#' {
            while x + 1 < width && grid[width + x + 1] == b'#' {
                x += 1;
            }
            chambers.push(Chamber {
                start_x,
                end_x: x,
                corridors: Vec::new(),
            });
            start_x = x;
        }
        x += 1;
    }

    // for each chamber, find all corridors
    for c in &mut chambers {
        for x in c.start_x..=c.start_x + (c.end_x - c.start_x) / 2 {
            if grid[(height / 2 - 1) * width + x] != b'#' {
                let (sulphur_up, length, end_x) = trace_corridor(x, true, &grid, width, height);
                let (sulphur_down, _, _) = trace_corridor(x, false, &grid, width, height);
                c.corridors.push(Cooridor {
                    start_x: x,
                    end_x,
                    length,
                    sulphur_up,
                    sulphur_down,
                });
            }
        }
    }

    chambers
}

fn trace_corridor(
    start_x: usize,
    up: bool,
    grid: &[u8],
    width: usize,
    height: usize,
) -> (usize, usize, usize) {
    let mut steps = 1;
    let mut sulphur = 0;
    let mut pos = (start_x, if up { height / 2 - 1 } else { height / 2 + 1 });
    let mut dir = if up { (0, -1) } else { (0, 1) };

    while pos.1 != height / 2 {
        if grid[pos.1 * width + pos.0] == b'@' {
            sulphur += 1;
        }
        for (dx, dy) in [dir, if up { (-dir.1, dir.0) } else { (dir.1, -dir.0) }] {
            let nx = pos.0 as i32 + dx;
            let ny = pos.1 as i32 + dy;
            if grid[ny as usize * width + nx as usize] != b'#' {
                pos = (nx as usize, ny as usize);
                dir = (dx, dy);
                steps += 1;
                break;
            }
        }
    }

    (sulphur, steps, pos.0)
}

fn collect_all(chamber: &Chamber) -> usize {
    // find all corridors with sulphur
    let corridors = chamber
        .corridors
        .iter()
        .flat_map(|c| {
            let mut r = Vec::new();
            if c.sulphur_up > 0 {
                r.push(c);
            }
            if c.sulphur_down > 0 {
                r.push(c);
            }
            r
        })
        .collect::<Vec<_>>();

    if corridors.is_empty() {
        // There are no corridors with sulphur. Just pass through the chamber.
        return chamber.end_x - chamber.start_x;
    }

    // run Dijsktra's to find the shortest path from the chamber's start to end
    // through all corridors with sulphur
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, 0u64, usize::MAX, true)));
    let mut best: HashMap<(u64, usize, bool), usize> = HashMap::new();

    while let Some(Reverse((steps, visited, corridor, is_corridor_start))) = queue.pop() {
        // first step: add the starts and ends of all corridors to the queue
        if steps == 0 {
            for (i, c) in corridors.iter().enumerate() {
                queue.push(Reverse((
                    c.start_x - chamber.start_x + c.length,
                    1 << i,
                    i,
                    false,
                )));
                queue.push(Reverse((
                    c.end_x - chamber.start_x + c.length,
                    1 << i,
                    i,
                    true,
                )));
            }
            continue;
        }

        // check if all corridors have been visited
        if visited == (1 << corridors.len()) - 1 {
            if corridor == usize::MAX {
                // we're at the chamber's end
                return steps;
            }

            // we still need to go to the chamber's end
            let x = if is_corridor_start {
                corridors[corridor].start_x
            } else {
                corridors[corridor].end_x
            };

            queue.push(Reverse((
                steps + (chamber.end_x - x),
                visited,
                usize::MAX,
                true,
            )));

            continue;
        }

        // go to the next unvisited chamber's start and end
        for (i, c) in corridors.iter().enumerate() {
            if visited & (1 << i) > 0 {
                continue;
            }

            let x = if is_corridor_start {
                corridors[corridor].start_x
            } else {
                corridors[corridor].end_x
            };

            let new_visited = visited | (1 << i);
            let new_steps_end = steps + c.start_x.abs_diff(x) + c.length;
            if *best.get(&(new_visited, i, false)).unwrap_or(&usize::MAX) > new_steps_end {
                best.insert((new_visited, i, false), new_steps_end);
                queue.push(Reverse((new_steps_end, new_visited, i, false)));
            }

            let new_steps_start = steps + c.end_x.abs_diff(x) + c.length;
            if *best.get(&(new_visited, i, true)).unwrap_or(&usize::MAX) > new_steps_start {
                best.insert((new_visited, i, true), new_steps_start);
                queue.push(Reverse((new_steps_start, new_visited, i, true)));
            }
        }
    }

    unreachable!()
}

/// Walk straight through the chamber
fn through_constants(chamber: &Chamber) -> [usize; 4] {
    [chamber.end_x - chamber.start_x, 0, 0, 0]
}

/// Walk to the start of the corridor, loop through the upper part 'n' times,
/// then take the upper part once more, and finally walk to the end of the
/// chamber
fn upper_constants(chamber: &Chamber, corridor: &Cooridor) -> [usize; 4] {
    let c1 =
        (corridor.start_x - chamber.start_x) + corridor.length + (chamber.end_x - corridor.end_x);
    let c2 = corridor.length + (corridor.end_x - corridor.start_x);
    let c3 = corridor.sulphur_up;
    let c4 = corridor.sulphur_up;
    [c1, c2, c3, c4]
}

/// Walk to the start of the corridor, loop through the lower part 'n' times,
/// then take the lower part once more, and finally walk to the end of the
/// chamber
fn lower_constants(chamber: &Chamber, corridor: &Cooridor) -> [usize; 4] {
    let c1 =
        (corridor.start_x - chamber.start_x) + corridor.length + (chamber.end_x - corridor.end_x);
    let c2 = corridor.length + (corridor.end_x - corridor.start_x);
    let c3 = corridor.sulphur_down;
    let c4 = corridor.sulphur_down;
    [c1, c2, c3, c4]
}

/// Walk to the start of the corridor, loop through the upper part AND the lower
/// parts 'n' times (in this order), then take the upper part once more, and
/// finally walk to the end of the chamber
fn upper_lower_constants(chamber: &Chamber, corridor: &Cooridor) -> [usize; 4] {
    let c1 =
        (corridor.start_x - chamber.start_x) + corridor.length + (chamber.end_x - corridor.end_x);
    let c2 = corridor.length * 2;
    let c3 = corridor.sulphur_up + corridor.sulphur_down;
    let c4 = corridor.sulphur_up;
    [c1, c2, c3, c4]
}

/// Walk to the start of the corridor, loop through the lower part AND the upper
/// parts 'n' times (in this order), then take the lower part once more, and
/// finally walk to the end of the chamber
fn lower_upper_constants(chamber: &Chamber, corridor: &Cooridor) -> [usize; 4] {
    let c1 =
        (corridor.start_x - chamber.start_x) + corridor.length + (chamber.end_x - corridor.end_x);
    let c2 = corridor.length * 2;
    let c3 = corridor.sulphur_down + corridor.sulphur_up;
    let c4 = corridor.sulphur_down;
    [c1, c2, c3, c4]
}

fn collect_constants(
    chambers: &[Chamber],
    constants: &mut Vec<[usize; 4]>,
    result: &mut Vec<Vec<[usize; 4]>>,
) {
    if constants.len() == chambers.len() {
        result.push(constants.clone());
        return;
    }

    let chamber = &chambers[constants.len()];

    constants.push(through_constants(chamber));
    collect_constants(chambers, constants, result);
    constants.pop();

    for corridor in &chamber.corridors {
        constants.push(upper_constants(chamber, corridor));
        collect_constants(chambers, constants, result);
        constants.pop();

        constants.push(lower_constants(chamber, corridor));
        collect_constants(chambers, constants, result);
        constants.pop();

        constants.push(upper_lower_constants(chamber, corridor));
        collect_constants(chambers, constants, result);
        constants.pop();

        constants.push(lower_upper_constants(chamber, corridor));
        collect_constants(chambers, constants, result);
        constants.pop();
    }
}

fn solve(chambers: Vec<Chamber>, target: usize, buffer: usize, exact: bool) -> u64 {
    // find loop with the best sulphur:steps ratio
    let mut max_ratio = 0.0;
    let mut max_ratio_chamber = 0;
    let mut max_ratio_constants = Vec::new();
    for (i, chamber) in chambers.iter().enumerate() {
        for corridor in &chamber.corridors {
            // upper part
            let r1 = corridor.sulphur_up as f64
                / (corridor.length + (corridor.end_x - corridor.start_x)) as f64;
            if r1 > max_ratio {
                max_ratio = r1;
                max_ratio_chamber = i;
                max_ratio_constants = vec![upper_constants(chamber, corridor)];
            }

            // lower part
            let r2 = corridor.sulphur_down as f64
                / (corridor.length + (corridor.end_x - corridor.start_x)) as f64;
            if r2 > max_ratio {
                max_ratio = r2;
                max_ratio_chamber = i;
                max_ratio_constants = vec![lower_constants(chamber, corridor)];
            }

            // upper + lower part
            let r3 =
                (corridor.sulphur_up + corridor.sulphur_down) as f64 / (corridor.length * 2) as f64;
            if r3 > max_ratio {
                max_ratio = r3;
                max_ratio_chamber = i;

                let cs_up = upper_lower_constants(chamber, corridor);
                let cs_lo = lower_upper_constants(chamber, corridor);
                if cs_up == cs_lo {
                    max_ratio_constants = vec![cs_up];
                } else {
                    max_ratio_constants = vec![cs_up, cs_lo];
                }
            }
        }
    }

    // collect all constants for all combinations of the corridors of the
    // remaining chambers
    let mut constants = Vec::new();
    let mut remaining_chambers = chambers.clone();
    remaining_chambers.remove(max_ratio_chamber);
    collect_constants(&remaining_chambers, &mut Vec::new(), &mut constants);

    // solve each equation and find the minimum `s`
    // s = c1_1 + a * c1_2 + c2_1 + b * c2_2 + c3_1 + c * c3_2
    // a * c1_3 + c1_4 + b * c2_3 + c2_4 + c * c3_3 + c3_4 = target
    let mut min = u64::MAX;
    for mrc in max_ratio_constants {
        for cs in &constants {
            let vrc = Int::new_const("vrc");
            let vars = (0..cs.len())
                .map(|i| Int::new_const(i as u32))
                .collect::<Vec<_>>();
            let s = cs.iter().enumerate().fold(
                &vrc * Int::from_u64(mrc[1] as u64) + Int::from_u64(mrc[0] as u64),
                |acc, (i, c)| {
                    acc + &vars[i] * Int::from_u64(c[1] as u64) + Int::from_u64(c[0] as u64)
                },
            );

            let l = cs.iter().enumerate().fold(
                &vrc * Int::from_u64(mrc[2] as u64) + Int::from_u64(mrc[3] as u64),
                |acc, (i, c)| {
                    acc + &vars[i] * Int::from_u64(c[2] as u64) + Int::from_u64(c[3] as u64)
                },
            );

            let lower_bound = target / mrc[2] - buffer;

            let opt = Optimize::new();
            if exact {
                opt.assert(l.eq(Int::from_u64(target as u64)));
            } else {
                opt.assert(l.ge(Int::from_u64(target as u64)));
            }
            opt.assert(vrc.ge(Int::from_u64(lower_bound as u64)));
            for v in &vars {
                opt.assert(v.ge(Int::from_u64(0)));
            }
            opt.minimize(&s);

            if opt.check(&[]) == SatResult::Sat {
                let m = opt.get_model().unwrap();
                let r = m.eval(&s, true).unwrap().as_u64().unwrap();
                min = min.min(r);
            }
        }
    }
    min
}

fn main() {
    // part 1
    let chambers = parse("everybody_codes_e2026_q15_p1.txt");

    // find shortest path in each chamber individually to reduce the problem
    // size
    let mut total = 0;
    for c in chambers {
        total += collect_all(&c);
    }
    println!("{total}");

    // part 2
    let chambers = parse("everybody_codes_e2026_q15_p2.txt");
    println!("{}", solve(chambers, 2026, 9, false));

    // part 3
    let chambers = parse("everybody_codes_e2026_q15_p3.txt");
    println!("{}", solve(chambers, 202620262026, 1, true));
}
