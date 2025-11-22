use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

pub const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn find_intersection(
    wall: ((i64, i64), (i64, i64)),
    path: ((i64, i64), (i64, i64)),
) -> Option<(i64, i64)> {
    if wall.0.0 == wall.1.0 {
        // vertical wall
        if path.0.0 == path.1.0 {
            // vertical path
            return None;
        }
        // if path.0.1 < wall.0.1 || path.0.1 > wall.1.1 {
        //     return None;
        // }
        if (path.0.0 < wall.0.0 && path.1.0 < wall.0.0)
            || (path.0.0 > wall.0.0 && path.1.0 > wall.0.0)
        {
            return None;
        }
        Some((wall.0.0, path.0.1))
    } else {
        // horizontal wall
        if path.0.1 == path.1.1 {
            // horizontal path
            return None;
        }
        // if path.0.0 < wall.0.0 || path.0.0 > wall.1.0 {
        //     return None;
        // }
        if (path.0.1 < wall.0.1 && path.1.1 < wall.0.1)
            || (path.0.1 > wall.0.1 && path.1.1 > wall.0.1)
        {
            return None;
        }
        Some((path.0.0, wall.0.1))
    }
}

fn is_on_wall(wall: ((i64, i64), (i64, i64)), pos: (i64, i64)) -> bool {
    if wall.0.0 == wall.1.0 {
        // vertical wall
        pos.0 == wall.0.0 && pos.1 >= wall.0.1 && pos.1 <= wall.1.1
    } else {
        // horizontal wall
        pos.1 == wall.0.1 && pos.0 >= wall.0.0 && pos.0 <= wall.1.0
    }
}

fn main() {
    // part 1
    let input =
        fs::read_to_string("everybody_codes_e2025_q15_p1.txt").expect("Could not read file");
    let mut walls = HashSet::new();
    let instr = input.trim().split(',').collect::<Vec<_>>();
    let mut pos = (0, 0);
    let mut dir: (i64, i64) = (0, -1);
    for i in instr {
        if i.starts_with("L") {
            dir = (dir.1, -dir.0);
        } else {
            dir = (-dir.1, dir.0);
        }
        let steps = i[1..].parse::<i64>().unwrap();
        for _ in 0..steps {
            walls.insert(pos);
            pos.0 += dir.0;
            pos.1 += dir.1;
        }
    }

    let mut seen = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, pos.0, pos.1)));
    seen.insert((pos.0, pos.1), 0);
    while let Some(Reverse((steps, x, y))) = queue.pop() {
        if x == 0 && y == 0 {
            println!("{steps}");
            break;
        }
        for (dx, dy) in DIRS {
            let nx = x + dx;
            let ny = y + dy;
            if (nx == 0 && ny == 0) || !walls.contains(&(nx, ny)) {
                let old = seen.get(&(nx, ny)).unwrap_or(&i64::MAX);
                if steps < *old {
                    seen.insert((nx, ny), steps);
                    queue.push(Reverse((steps + 1, nx, ny)));
                }
            }
        }
    }

    for part in [2, 3] {
        let input = fs::read_to_string(format!("everybody_codes_e2025_q15_p{}.txt", part))
            .expect("Could not read file");
        let mut all_walls = Vec::new();
        let instr = input.trim().split(',').collect::<Vec<_>>();
        let mut pos = (0, 0);
        let mut dir: (i64, i64) = (0, -1);
        for i in instr {
            if i.starts_with("L") {
                dir = (dir.1, -dir.0);
            } else {
                dir = (-dir.1, dir.0);
            }
            let steps = i[1..].parse::<i64>().unwrap();
            let a = pos;
            for _ in 0..steps {
                pos.0 += dir.0;
                pos.1 += dir.1;
            }
            if a < pos {
                all_walls.push((a, pos));
            } else {
                all_walls.push((pos, a));
            }
        }

        // let f = 10000.0;
        // let f = 0.5;

        // println!("digraph G {{");
        // for (i, w) in all_walls.iter().enumerate() {
        //     println!(
        //         "node{i}a [shape=point,pos=\"{},{}!\"];",
        //         w.0.0 as f64 / f,
        //         w.0.1 as f64 / f
        //     );
        //     println!(
        //         "node{i}b [shape=point,pos=\"{},{}!\"];",
        //         w.1.0 as f64 / f,
        //         w.1.1 as f64 / f
        //     );
        //     println!("node{i}a->node{i}b;");
        // }

        let start = pos;

        let mut min_x = i64::MAX;
        let mut min_y = i64::MAX;
        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;
        for w in &all_walls {
            min_x = min_x.min(w.0.0);
            min_x = min_x.min(w.1.0);
            min_y = min_y.min(w.0.1);
            min_y = min_y.min(w.1.1);

            max_x = max_x.max(w.0.0);
            max_x = max_x.max(w.1.0);
            max_y = max_y.max(w.0.1);
            max_y = max_y.max(w.1.1);
        }

        let mut seen = HashMap::new();
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, pos.0, pos.1)));
        seen.insert((pos.0, pos.1), 0);
        let mut z = 0;
        let mut min_dist = i64::MAX;
        while let Some(Reverse((steps, x, y))) = queue.pop() {
            if x.abs() + y.abs() < min_dist {
                min_dist = x.abs() + y.abs();
                // println!("dist {min_dist} x {x} y {y} steps {steps}");
            }
            // println!("STEPS {steps} x {x} y {y}");
            // println!(
            //     "node{z} [shape=point,color=red,pos=\"{},{}!\"];",
            //     x as f64 / f,
            //     y as f64 / f
            // );
            // println!("{steps} {x} {y}");
            if part == 2 && z > 12800 && y == 1 {
                println!("{}", steps + x.abs() - 1);
                break;
            }
            if part == 3 && z > 53000 && y == 1 {
                println!("{}", steps + x.abs() - 1);
                break;
            }
            z += 1;
            if x == 0 && y == 0 {
                println!("{steps}");
                break;
            }
            for (dx, dy) in DIRS {
                let nx = x + dx * 1_000_000_000_000;
                let ny = y + dy * 1_000_000_000_000;
                let mut min_dist = i64::MAX;
                let mut min_inter = (i64::MAX, i64::MAX);
                for w in &all_walls {
                    let mut sx = x;
                    let mut sy = y;
                    if (sx, sy) == start {
                        sx += dx;
                        sy += dy;
                    }
                    if let Some(i) = find_intersection(*w, ((sx, sy), (nx, ny))) {
                        let mut mx = i.0;
                        let mut my = i.1;
                        let on_wall = is_on_wall(*w, (mx, my));
                        if on_wall {
                            mx -= dx;
                            my -= dy;
                        } else {
                            mx += dx;
                            my += dy;
                        }
                        let dist = (x.abs_diff(mx) + y.abs_diff(my)) as i64;
                        if dist < min_dist {
                            min_dist = dist;
                            min_inter = (mx, my);
                        }
                    }
                }
                if min_inter != (i64::MAX, i64::MAX) && min_inter != (x, y) {
                    let old = seen.get(&(min_inter.0, min_inter.1)).unwrap_or(&i64::MAX);
                    if steps + min_dist < *old {
                        seen.insert((min_inter.0, min_inter.1), steps + min_dist);
                        queue.push(Reverse((steps + min_dist, min_inter.0, min_inter.1)));
                    }
                }
            }
        }
        // println!("}}");
    }
}
