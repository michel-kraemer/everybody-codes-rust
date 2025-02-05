use std::collections::HashSet;
use std::fs;

use bucket_queue::BucketQueue;

mod bucket_queue;

#[derive(Clone, Copy)]
struct Edge {
    dist: usize,
    to: usize,
}

struct Star {
    x: usize,
    y: usize,
}

impl Star {
    fn dist(&self, other: &Star) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn main() {
    for part in [1, 2, 3] {
        let input = fs::read_to_string(format!("everybody_codes_e2024_q17_p{}.txt", part)).unwrap();

        // collect all stars
        let mut stars = Vec::new();
        for (y, row) in input.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '*' {
                    stars.push(Star { x, y });
                }
            }
        }

        // find edges between stars (for part 3, only consider edges shorter than 6)
        let mut edges = vec![Vec::new(); stars.len()];
        let mut max_dist = 0;
        for i in 0..stars.len() {
            for j in i + 1..stars.len() {
                let d = stars[i].dist(&stars[j]);
                if part != 3 || d < 6 {
                    max_dist = max_dist.max(d);
                    edges[i].push(Edge { dist: d, to: j });
                    edges[j].push(Edge { dist: d, to: i });
                }
            }
        }

        let mut remaining_stars = (0..stars.len()).collect::<HashSet<_>>();
        let mut in_constellation = vec![false; stars.len()];
        let mut shortest_dist = vec![usize::MAX; stars.len()];
        let mut constellation_sizes = Vec::new();

        while !remaining_stars.is_empty() {
            // start with any of the remaining stars
            let start = *remaining_stars.iter().next().unwrap();
            remaining_stars.remove(&start);

            // perform Prim's algorithm to construct minimum spanning tree
            let mut bucket_queue = BucketQueue::new(max_dist);
            bucket_queue.insert(0, start);

            let mut constellation: Vec<usize> = Vec::new();
            shortest_dist[start] = 0;

            while let Some(s) = bucket_queue.pop() {
                if in_constellation[s] {
                    // shortest distance has already been calculated
                    continue;
                }

                // add s to constellation and remove it from remaining stars
                constellation.push(s);
                remaining_stars.remove(&s);
                in_constellation[s] = true;

                // for each neighbor of s ...
                for &e in &edges[s] {
                    // ... if it's not in a constellation yet and its current
                    // shortest distance is greater than its distance to s
                    if !in_constellation[e.to] && shortest_dist[e.to] > e.dist {
                        // ... update its shortest distance and add it to the queue
                        shortest_dist[e.to] = e.dist;
                        bucket_queue.insert(e.dist - 1, e.to);
                    }
                }
            }

            // calculate constellation size
            let mut sum = 0;
            for &s in &constellation {
                sum += shortest_dist[s];
            }
            constellation_sizes.push(sum + constellation.len());
        }

        constellation_sizes.sort_unstable();

        if part == 3 {
            // product of the 3 largest constellations
            println!(
                "{}",
                constellation_sizes
                    .into_iter()
                    .rev()
                    .take(3)
                    .product::<usize>()
            );
        } else {
            println!("{}", constellation_sizes[0]);
        }
    }
}
