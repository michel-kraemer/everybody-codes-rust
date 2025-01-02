use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() {
    for part in [1, 2, 3] {
        let input = fs::read_to_string(format!("everybody_codes_e2024_q06_p{}.txt", part))
            .expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();

        // read map
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        for l in lines {
            let (left, right) = l.split_once(":").unwrap();
            for r in right.split(",") {
                map.entry(left).or_default().push(r);
            }
        }

        // flood fill to find all paths
        let mut paths = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(("RR", Vec::new())); // RR is the root

        while let Some((cur, mut path)) = queue.pop_front() {
            if part == 3 && (cur == "ANT" || cur == "BUG") {
                // skip ants and bugs in part 3
                continue;
            }

            path.push(cur);
            if cur == "@" {
                // reached a fruit
                paths.push(path);
                continue;
            }

            // add all neighbors to queue
            if let Some(ns) = map.get(cur) {
                for n in ns {
                    queue.push_back((n, path.clone()));
                }
            }
        }

        // find the only path that has a unique length
        paths.sort_unstable_by_key(|p| p.len());
        let mut i = 0;
        let most_powerful_branch = loop {
            let pl = paths[i].len();
            let prevl = if i > 0 { paths[i - 1].len() } else { pl };
            let nextl = if i < paths.len() - 1 {
                paths[i + 1].len()
            } else {
                pl
            };
            if pl != prevl && pl != nextl {
                break &paths[i];
            }
            i += 1;
        };

        if part == 1 {
            println!("{}", most_powerful_branch.join(""));
        } else {
            println!(
                "{}",
                most_powerful_branch
                    .iter()
                    .map(|s| s.chars().next().unwrap())
                    .collect::<String>()
            );
        }
    }
}
