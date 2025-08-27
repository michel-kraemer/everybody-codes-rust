use std::fs;

struct Node<'a> {
    rank: u64,
    symbol: &'a str,
    left: Option<(usize, usize)>,
    right: Option<(usize, usize)>,
}

fn insert(child: (usize, usize), nodes: &mut [Vec<Node<'_>>]) {
    let mut r = (child.0, 0);
    loop {
        if nodes[child.0][child.1].rank < nodes[r.0][r.1].rank {
            if let Some(li) = nodes[r.0][r.1].left {
                r = li;
            } else {
                nodes[r.0][r.1].left = Some(child);
                break;
            }
        } else if let Some(ri) = nodes[r.0][r.1].right {
            r = ri;
        } else {
            nodes[r.0][r.1].right = Some(child);
            break;
        }
    }
}

fn get_max_level(cur: (usize, usize), nodes: &[Vec<Node<'_>>], cur_level: usize) -> usize {
    let mut result = cur_level;
    if let Some(l) = nodes[cur.0][cur.1].left {
        result = result.max(get_max_level(l, nodes, cur_level + 1));
    }
    if let Some(r) = nodes[cur.0][cur.1].right {
        result = result.max(get_max_level(r, nodes, cur_level + 1));
    }
    result
}

fn read_nodes(
    cur: (usize, usize),
    nodes: &[Vec<Node<'_>>],
    cur_level: usize,
    level: usize,
    result: &mut String,
) {
    if cur_level == level {
        result.push_str(nodes[cur.0][cur.1].symbol);
        return;
    }
    if let Some(l) = nodes[cur.0][cur.1].left {
        read_nodes(l, nodes, cur_level + 1, level, result);
    }
    if let Some(r) = nodes[cur.0][cur.1].right {
        read_nodes(r, nodes, cur_level + 1, level, result);
    }
}

fn main() {
    for part in 1..=3 {
        let input = fs::read_to_string(format!("everybody_codes_e1_q02_p{}.txt", part))
            .expect("Could not read file");

        let mut nodes: Vec<Vec<Node>> = vec![Vec::new(), Vec::new()];
        for l in input.lines() {
            if l.starts_with("SWAP") {
                let (_, id) = l.split_once(" ").unwrap();
                let id = id.parse::<usize>().unwrap();
                let (left, right) = nodes.split_at_mut(1);
                std::mem::swap(&mut left[0][id - 1].rank, &mut right[0][id - 1].rank);
                std::mem::swap(&mut left[0][id - 1].symbol, &mut right[0][id - 1].symbol);
                if part == 3 {
                    std::mem::swap(&mut left[0][id - 1].left, &mut right[0][id - 1].left);
                    std::mem::swap(&mut left[0][id - 1].right, &mut right[0][id - 1].right);
                }
                continue;
            }

            let parts = l.split_whitespace().collect::<Vec<_>>();
            let (_, id) = parts[1].split_once("=").unwrap();
            let id = id.parse::<usize>().unwrap();

            for tree in 0..2 {
                let (_, t) = parts[tree + 2].split_once("=").unwrap();
                let (rank, symbol) = t[1..t.len() - 1].split_once(",").unwrap();
                let rank = rank.parse::<u64>().unwrap();

                nodes[tree].push(Node {
                    rank,
                    symbol,
                    left: None,
                    right: None,
                });
            }

            if id > 1 {
                insert((0, id - 1), &mut nodes);
                insert((1, id - 1), &mut nodes);
            }
        }

        for tree in 0..2 {
            let max_level = get_max_level((tree, 0), &nodes, 0);
            let mut max_result = "".to_string();
            for i in 0..=max_level {
                let mut result = String::new();
                read_nodes((tree, 0), &nodes, 0, i, &mut result);
                if result.len() > max_result.len() {
                    max_result = result;
                }
            }
            print!("{}", max_result);
        }

        println!();
    }
}
