use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct Incoming {
    thickness: i64,
    source: Option<usize>,
}

#[derive(Debug, Clone)]
struct Plant {
    id: i64,
    free: bool,
    thickness: i64,
    incoming: Vec<Incoming>,
}

fn dfs(id: usize, plants: &[Plant]) -> i64 {
    let mut result = if plants[id].free {
        plants[id].thickness
    } else {
        0
    };
    for i in &plants[id].incoming {
        if let Some(src) = i.source {
            result += dfs(src, plants) * i.thickness;
        }
    }
    if plants[id].thickness > result {
        0
    } else {
        result
    }
}

fn dfs2(
    plants: &[Plant],
    mask: u128,
    remaining_bits: &[(&usize, &Vec<i64>)],
    maxes: &HashMap<i64, i64>,
    the_max: &mut i64,
) {
    if remaining_bits.is_empty() {
        let mut plants_copy = plants.to_vec();
        for i in 0..81 {
            let th = if mask & (1 << i) > 0 { 1 } else { 0 };
            plants_copy[i].thickness = th;
        }
        let r = dfs(plants_copy.len() - 1, &plants_copy);
        if r > *the_max {
            *the_max = r;
            for i in 0..81 {
                let th = if mask & (1 << i) > 0 { 1 } else { 0 };
                print!("{th} ");
            }
            println!("{r}");
        }
        return;
    }

    for v in [0, 1] {
        let bit_id = remaining_bits[0].0;
        if remaining_bits[0].1.len() > 1 {
            dfs2(
                plants,
                mask | (v << (bit_id - 1)),
                &remaining_bits[1..],
                maxes,
                the_max,
            );
        } else {
            let pid = remaining_bits[0].1[0];
            let max = *maxes.get(&pid).unwrap();
            let mut remaining_max = max;
            let mut new_mask = mask | (v << (bit_id - 1));

            for i in &plants[pid as usize - 1].incoming {
                if i.source == Some(*bit_id - 1) {
                    if i.thickness > 0 && v == 0 {
                        remaining_max -= i.thickness;
                    } else if i.thickness < 0 && v == 1 {
                        remaining_max -= -i.thickness;
                    }
                }
            }

            let mut rb2 = remaining_bits[1..].to_vec();
            let mut j = 0;
            while j < rb2.len() {
                if rb2[j].1 == remaining_bits[0].1 {
                    let other_bit_id = rb2[j].0;
                    for i in &plants[pid as usize - 1].incoming {
                        if i.source == Some(*other_bit_id - 1) {
                            let mut must_filter = false;
                            if i.thickness > 0
                                && remaining_max - i.thickness < plants[pid as usize - 1].thickness
                            {
                                new_mask |= 1 << (other_bit_id - 1);
                                must_filter = true;
                            } else if i.thickness < 0
                                && remaining_max + i.thickness < plants[pid as usize - 1].thickness
                            {
                                must_filter = true;
                            }
                            if must_filter {
                                if pid == 92 && *bit_id == 2 && v == 0 {
                                    println!("REMOVE {:?}", rb2[j]);
                                }
                                rb2.remove(j);
                            } else {
                                j += 1;
                            }
                            break;
                        }
                    }
                } else {
                    j += 1;
                }
            }

            if pid == 92 && *bit_id == 2 && v == 0 {
                println!(
                    "FOUND {max} {remaining_max} {} {}",
                    remaining_bits.len() - 1,
                    rb2.len()
                );
            }

            dfs2(plants, new_mask, &rb2, maxes, the_max);
        }
    }
}

fn parse(path: &str) -> (Vec<Plant>, String) {
    let input = fs::read_to_string(path).expect("Could not read file");
    let (left, right) = if let Some((l, r)) = input.trim().split_once("\n\n\n") {
        (l, r)
    } else {
        (input.trim(), "")
    };
    let blocks = left.split("\n\n").collect::<Vec<_>>();

    let mut plants = Vec::new();
    for b in blocks {
        let lines = b.lines().collect::<Vec<_>>();
        let plant = lines[0].split(' ').collect::<Vec<_>>()[1]
            .parse::<i64>()
            .unwrap();
        let thickness = lines[0][0..lines[0].len() - 1]
            .split(' ')
            .collect::<Vec<_>>()[4]
            .parse::<i64>()
            .unwrap();
        if lines[1].starts_with("- free") {
            let incoming_thickness = lines[1].split(' ').collect::<Vec<_>>()[5]
                .parse::<i64>()
                .unwrap();
            // free plant
            plants.push(Plant {
                id: plant,
                free: true,
                thickness,
                incoming: vec![Incoming {
                    thickness: incoming_thickness,
                    source: None,
                }],
            });
        } else {
            let mut incoming = Vec::new();
            for l in &lines[1..] {
                let source = l.split(' ').collect::<Vec<_>>()[4].parse::<i64>().unwrap();
                let incoming_thickness =
                    l.split(' ').collect::<Vec<_>>()[7].parse::<i64>().unwrap();
                incoming.push(Incoming {
                    thickness: incoming_thickness,
                    source: Some((source - 1) as usize),
                });
            }
            plants.push(Plant {
                id: plant,
                free: false,
                thickness,
                incoming,
            });
        }
    }

    (plants, right.to_string())
}

fn main() {
    // part 1
    let (plants, _) = parse("everybody_codes_e2025_q18_p1.txt");
    println!("{}", dfs(plants.len() - 1, &plants));

    // part 2
    let (plants, right) = parse("everybody_codes_e2025_q18_p2.txt");
    let mut total = 0;
    for t in right.lines() {
        let thicknesses = t
            .split(' ')
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut plants_copy = plants.clone();
        for (j, th) in thicknesses.iter().enumerate() {
            plants_copy[j].thickness = *th;
        }
        total += dfs(plants_copy.len() - 1, &plants_copy);
    }
    println!("{total}");

    // part 3
    let (plants, right) = parse("everybody_codes_e2025_q18_p3.txt");

    // part 1
    // let mut candidates = (0..plants.len()).collect::<HashSet<_>>();
    // for p in &plants {
    //     for i in &p.incoming {
    //         if let Some(src) = i.source {
    //             candidates.remove(&src);
    //         }
    //     }
    // }
    // println!("{candidates:?}");
    // // assert_eq!(candidates.len(), 1);

    // println!("{}", dfs(18, &plants));

    let mut first_level_plants = Vec::new();
    for p in &plants {
        if !p.free && plants[p.incoming[0].source.unwrap()].free {
            first_level_plants.push(p.id);
        }
    }

    let mut maxes = HashMap::new();
    for id in &first_level_plants {
        let p = &plants[*id as usize - 1];
        let max = p
            .incoming
            .iter()
            .filter(|p| p.thickness > 0)
            .map(|p| p.thickness)
            .sum::<i64>();
        if max >= p.thickness {
            maxes.insert(*id, max);
        }
    }

    let mut bits: HashMap<usize, Vec<i64>> = HashMap::new();
    for (id, m) in &maxes {
        for i in &plants[*id as usize - 1].incoming {
            bits.entry(i.source.unwrap() + 1).or_default().push(*id);
        }
    }

    let mut zeroes = HashSet::new();
    for (bit_id, plant_ids) in &bits {
        let mut n_would_die = 0;
        for pid in plant_ids {
            let max = maxes.get(pid).unwrap();
            for i in &plants[*pid as usize - 1].incoming {
                if i.source == Some(*bit_id - 1)
                    && i.thickness < 0
                    && max + i.thickness < plants[*pid as usize - 1].thickness
                {
                    n_would_die += 1;
                }
            }
        }
        if n_would_die == plant_ids.len() {
            zeroes.insert(*bit_id);
        }
    }

    let mut ones = HashSet::new();
    for (bit_id, plant_ids) in &bits {
        let mut n_would_die = 0;
        for pid in plant_ids {
            let max = maxes.get(pid).unwrap();
            for i in &plants[*pid as usize - 1].incoming {
                if i.source == Some(*bit_id - 1)
                    && i.thickness > 0
                    && max - i.thickness < plants[*pid as usize - 1].thickness
                {
                    n_would_die += 1;
                }
            }
        }
        if n_would_die == plant_ids.len() {
            ones.insert(*bit_id);
        }
    }

    let mut remaining_bits = bits
        .iter()
        .filter(|(b, _)| !ones.contains(b) && !zeroes.contains(b))
        .collect::<Vec<_>>();
    remaining_bits.sort();

    let mut mask: u128 = 0;
    for o in &ones {
        mask |= 1 << (*o - 1);
    }

    // let mut the_max = 0;
    // dfs2(&plants, mask, &remaining_bits, &maxes, &mut the_max);

    let max = 15489;

    let mut total = 0;
    for t in right.lines() {
        let thicknesses = t
            .split(' ')
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut plants_copy = plants.clone();
        for (j, th) in thicknesses.iter().enumerate() {
            plants_copy[j].thickness = *th;
        }
        let r = dfs(plants_copy.len() - 1, &plants_copy);
        if r > 0 {
            total += max - r;
        }
    }
    println!("{total}");
}
