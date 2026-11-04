use std::collections::{HashMap, HashSet};
use std::fs;

fn parse(path: &str) -> HashMap<u64, (u64, Vec<u64>)> {
    let input = fs::read_to_string(path).expect("Could not read file");
    let mut clouds = HashMap::new();
    for l in input.lines() {
        let mut parts = l.split(", ");
        let id = parts.next().unwrap();
        let energy = parts.next().unwrap();
        let connections = parts.next().unwrap();
        let (_, id) = id.split_once('=').unwrap();
        let (_, energy) = energy.split_once('=').unwrap();
        let (_, connections) = connections.split_once('=').unwrap();
        let id = id.parse::<u64>().unwrap();
        let energy = energy.parse::<u64>().unwrap();
        let connections = connections
            .split(',')
            .map(|c| c.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        clouds.insert(id, (energy, connections.to_vec()));
    }
    clouds
}

fn compute(
    start_id: u64,
    start_energy: u64,
    limit: u64,
    mut clouds: HashMap<u64, (u64, Vec<u64>)>,
) -> u64 {
    let mut exploded = HashSet::new();

    // map (destination, energy) to number of pulses with the same destination
    // and energy
    let mut pulses = HashMap::new();
    pulses.insert((start_id, start_energy), 1);

    while !pulses.is_empty() {
        // propagate pulses
        let mut new_pulses: HashMap<(u64, u64), u64> = HashMap::new();
        for ((dest, energy), num) in pulses {
            // increase energy of destination
            let c = clouds.get_mut(&dest).unwrap();
            c.0 += energy * num;

            // forward energy
            if energy > 1 {
                for &o in &c.1 {
                    *new_pulses.entry((o, energy - 1)).or_default() += num;
                }
            }
        }

        // find exploded clouds and create new pulses if they've just exploded
        for (id, (energy, connections)) in &clouds {
            if *energy >= limit && exploded.insert(*id) {
                for &o in connections {
                    *new_pulses.entry((o, start_energy)).or_default() += 1;
                }
            }
        }

        // retain only those pulses that will arrive at clouds that are not
        // exploded
        new_pulses.retain(|k, _| !exploded.contains(&k.0));

        pulses = new_pulses;
    }

    // compute total energy
    let mut total = 0;
    for (energy, _) in clouds.values() {
        if *energy < limit {
            total += energy;
        }
    }

    total
}

fn main() {
    // part 1
    let clouds = parse("everybody_codes_e2026_q07_p1.txt");
    let max_id = *clouds.iter().max_by_key(|c| c.1.1.len()).unwrap().0;
    println!("{}", compute(max_id, 5, u64::MAX, clouds));

    // part 2
    let clouds = parse("everybody_codes_e2026_q07_p2.txt");
    let max_id = *clouds.iter().max_by_key(|c| c.1.1.len()).unwrap().0;
    println!("{}", compute(max_id, 10, u64::MAX, clouds));

    // part 3
    let clouds = parse("everybody_codes_e2026_q07_p3.txt");
    println!(
        "{}",
        clouds
            .keys()
            .map(|&c| compute(c, 10, 1_000_000, clouds.clone()))
            .max()
            .unwrap()
    );
}
