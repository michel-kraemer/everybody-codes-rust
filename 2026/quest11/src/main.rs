use std::fs;

fn parse(path: &str) -> (usize, Vec<(usize, usize)>) {
    let input = fs::read_to_string(path).expect("Could not read file");
    let (portals, rest) = input.split_once("\n\n").unwrap();
    let portals = portals[9..].parse::<usize>().unwrap();
    let mut additional_connections = Vec::new();
    for r in rest.lines() {
        let (a, b) = r.split_once(' ').unwrap();
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();
        additional_connections.push((a, b));
    }
    (portals, additional_connections)
}

/// Create a list of all possible pairings
fn pairs(
    list: &mut [usize],
    start: usize,
    tmp: &mut Vec<(usize, usize)>,
    result: &mut Vec<Vec<(usize, usize)>>,
) {
    if tmp.len() == list.len() / 2 {
        result.push(tmp.to_vec());
        return;
    }
    for i in start..list.len() {
        let a = list[i];
        if a != usize::MAX {
            list[i] = usize::MAX;
            for j in i + 1..list.len() {
                let b = list[j];
                if b != usize::MAX {
                    list[j] = usize::MAX;
                    tmp.push((a, b));
                    pairs(list, i + 1, tmp, result);
                    tmp.pop();
                    list[j] = b;
                }
            }
            list[i] = a;
        }
    }
}

/// Compute the sum of all shortest paths from the given portal to any other
/// portal using the given connections
fn shortest(from: usize, connections: &[Vec<usize>], portals: usize) -> usize {
    let mut min_steps = vec![usize::MAX; portals];
    min_steps[from] = 0;
    let mut queue = Vec::new();
    queue.push((from, 0));
    while let Some((pos, steps)) = queue.pop() {
        for &n in &connections[pos] {
            if min_steps[n] > steps + 1 {
                min_steps[n] = steps + 1;
                queue.push((n, steps + 1));
            }
        }
    }
    min_steps.iter().sum()
}

fn main() {
    // part 1
    let (portals, additional_connections) = parse("everybody_codes_e2026_q11_p1.txt");

    let mut connections = vec![Vec::new(); portals];
    for (i, c) in connections.iter_mut().enumerate() {
        c.push((i + 1) % portals);
        c.push((i + portals - 1) % portals);
    }
    let before = shortest(0, &connections, portals) * portals;

    for ac in additional_connections {
        connections[ac.0].push(ac.1);
        connections[ac.1].push(ac.0);
    }

    let mut after = 0;
    for i in 0..portals {
        after += shortest(i, &connections, portals);
    }
    println!("{}", before - after);

    // part 2
    let (portals, additional_connections) = parse("everybody_codes_e2026_q11_p2.txt");

    let mut connections = vec![Vec::new(); portals];
    for (i, c) in connections.iter_mut().enumerate() {
        for j in 1..4 {
            c.push((i + j) % portals);
            c.push((i + portals - j) % portals);
        }
    }
    let before = shortest(0, &connections, portals) * portals;

    for ac in additional_connections {
        connections[ac.0].push(ac.1);
        connections[ac.1].push(ac.0);
    }

    let mut min = usize::MAX;
    for a in 0..portals {
        for b in 0..portals {
            if a != b && !connections[a].contains(&b) {
                let mut cl = connections.clone();
                cl[a].push(b);
                cl[b].push(a);

                let mut sum = 0;
                for i in 0..portals {
                    sum += shortest(i, &cl, portals);
                }
                min = min.min(sum);
            }
        }
    }
    println!("{}", before - min);

    // part 3
    let (portals, additional_connections) = parse("everybody_codes_e2026_q11_p3.txt");

    let mut connections = vec![Vec::new(); portals];
    for (i, c) in connections.iter_mut().enumerate() {
        for j in 1..4 {
            c.push((i + j) % portals);
            c.push((i + portals - j) % portals);
        }
    }
    let before = shortest(0, &connections, portals) * portals;

    for ac in additional_connections {
        connections[ac.0].push(ac.1);
        connections[ac.1].push(ac.0);
    }

    let mut pairings = Vec::new();
    pairs(&mut Vec::from_iter(0..8), 0, &mut Vec::new(), &mut pairings);

    let mut min = usize::MAX;
    for start in 0..portals / 8 {
        let mut crystals = Vec::new();
        for i in 0..8 {
            crystals.push(start + i * portals / 8);
        }

        for pairing in &pairings {
            let mut cl = connections.clone();
            let mut ok = true;
            for p in pairing {
                if connections[crystals[p.0]].contains(&crystals[p.1]) {
                    ok = false;
                    break;
                }
                cl[crystals[p.0]].push(crystals[p.1]);
                cl[crystals[p.1]].push(crystals[p.0]);
            }

            if ok {
                let mut sum = 0;
                for i in 0..portals {
                    sum += shortest(i, &cl, portals);
                }
                min = min.min(sum);
            }
        }
    }
    println!("{}", before - min);
}
