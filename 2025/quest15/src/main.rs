use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

pub const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

type Wall = ((i64, i64), (i64, i64));

/// Sort the given vertices using a key extraction function. Then iterate
/// through all n sorted unique keys and map them to the interval [0,n).
///
/// Example:
/// * Unsorted keys: `[2, 12, 2, 4, 4, 5, 0, 7, 8, 4, 2, 0]`
/// * Sorted unique keys: `[0, 2, 4, 5, 7, 8, 12]`
/// * Result: `{0: 0, 2: 1, 4: 2, 5: 3, 7: 4, 8: 5, 12: 6}`
fn compress_coordinates_by<F>(vertices: &mut Vec<(i64, i64)>, mut f: F) -> HashMap<i64, i64>
where
    F: FnMut(&(i64, i64)) -> i64,
{
    vertices.sort_unstable_by_key(&mut f);
    let mut last = i64::MAX;
    let mut new = 0;
    let mut map = HashMap::new();
    for v in vertices {
        let vv = f(v);
        if vv != last {
            map.insert(vv, new);
            last = vv;
            new += 1;
        }
    }
    map
}

/// Reverse the given mapping and insert a left and a right border around each
/// coordinate.
///
/// Example:
/// ```text
/// {    0: 0,     2: 1,     4: 2,     5: 3,     7: 4,     8: 5,      12: 6 }
///      |         |         |         |         |         |           |
/// [-1, 0, 1,  1, 2, 3,  3, 4, 5,  4, 5, 6,  6, 7, 8,  7, 8, 9,  11, 12, 13]
/// ```
fn revmap(map: &HashMap<i64, i64>) -> Vec<i64> {
    let mut result = vec![0; map.len() * 3];
    for (&old, &new) in map {
        result[new as usize * 3] = old - 1;
        result[new as usize * 3 + 1] = old;
        result[new as usize * 3 + 2] = old + 1;
    }
    result
}

/// Compress the given list of walls. Map the n unique x values to the interval
/// [0,n) and the m unique y values to the interval [0,m). Then add a small
/// border to the left and right of each value so there is space for us to
/// travel through. Return the compressed walls and a mapping from compressed
/// coordinates to real coordinates.
fn compress(mut walls: Vec<Wall>) -> (Vec<Wall>, Vec<i64>, Vec<i64>) {
    let mut vertices = walls.iter().map(|w| w.1).collect::<Vec<_>>();
    vertices.push(walls[0].0);

    // compress coordinates
    let xmap = compress_coordinates_by(&mut vertices, |(x, _)| *x);
    let ymap = compress_coordinates_by(&mut vertices, |(_, y)| *y);

    // translate coordinates
    for w in walls.iter_mut() {
        w.0.0 = xmap[&w.0.0] * 3 + 1;
        w.0.1 = ymap[&w.0.1] * 3 + 1;
        w.1.0 = xmap[&w.1.0] * 3 + 1;
        w.1.1 = ymap[&w.1.1] * 3 + 1;
    }

    // create reverse mapping
    let xmaprev = revmap(&xmap);
    let ymaprev = revmap(&ymap);

    (walls, xmaprev, ymaprev)
}

fn main() {
    for part in 1..=3 {
        // parse
        let input = fs::read_to_string(format!("everybody_codes_e2025_q15_p{part}.txt"))
            .expect("Could not read file");
        let instructions = input.trim().split(',').collect::<Vec<_>>();

        // parse walls to lines
        let mut pos = (0, 0);
        let mut dir: (i64, i64) = (0, -1);
        let mut walls = Vec::new();
        for i in instructions {
            if i.starts_with('L') {
                dir = (dir.1, -dir.0);
            } else {
                dir = (-dir.1, dir.0);
            }
            let steps = i[1..].parse::<i64>().unwrap();

            let a = pos;
            pos.0 += dir.0 * steps;
            pos.1 += dir.1 * steps;
            walls.push((a, pos));
        }

        // compress walls
        let (walls, xmaprev, ymaprev) = compress(walls);
        let width = xmaprev.len() as i64;
        let height = ymaprev.len() as i64;

        // create grid and draw compressed walls into it
        let mut grid = vec![b'.'; (width * height) as usize];
        for w in &walls {
            if w.0.0 == w.1.0 {
                // vertical wall
                let sy = w.0.1.min(w.1.1);
                let ey = w.0.1.max(w.1.1);
                for y in sy..=ey {
                    grid[(y * width + w.0.0) as usize] = b'#';
                }
            } else {
                // horizontal wall
                let sx = w.0.0.min(w.1.0);
                let ex = w.0.0.max(w.1.0);
                for x in sx..=ex {
                    grid[(w.0.1 * width + x) as usize] = b'#';
                }
            }
        }

        // perform Dijkstra's on the compressed grid but count steps in real world
        let mut seen = vec![i64::MAX; grid.len()];
        let mut queue = BinaryHeap::new();
        let dest = walls[0].0;
        let pos = walls[walls.len() - 1].1;
        queue.push(Reverse((0, pos.0, pos.1)));
        seen[(pos.1 * width + pos.0) as usize] = 0;
        while let Some(Reverse((steps, x, y))) = queue.pop() {
            let realx = xmaprev[x as usize];
            let realy = ymaprev[y as usize];
            if realx == 0 && realy == 0 {
                println!("{steps}");
                break;
            }

            for (dx, dy) in DIRS {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0
                    && nx < width
                    && ny >= 0
                    && ny < height
                    && ((nx, ny) == dest || grid[(ny * width + nx) as usize] != b'#')
                {
                    let realnx = xmaprev[nx as usize];
                    let realny = ymaprev[ny as usize];
                    let dist = (realnx - realx).abs() + (realny - realy).abs();
                    if steps + dist < seen[(ny * width + nx) as usize] {
                        seen[(ny * width + nx) as usize] = steps + dist;
                        queue.push(Reverse((steps + dist, nx, ny)));
                    }
                }
            }
        }
    }
}
