use std::fs;

struct FamilyMember {
    id: usize,
    parent: usize,
    size: usize,
    dna: Vec<u8>,
    similarity: u64,
}

fn find(x: usize, nodes: &mut [FamilyMember]) -> usize {
    if nodes[x].parent != x {
        nodes[x].parent = find(nodes[x].parent, nodes);
        nodes[x].parent
    } else {
        x
    }
}

fn union(mut x: usize, mut y: usize, nodes: &mut [FamilyMember]) {
    x = find(x, nodes);
    y = find(y, nodes);

    if x == y {
        return;
    }

    if nodes[x].size < nodes[y].size {
        std::mem::swap(&mut x, &mut y);
    }

    nodes[y].parent = x;
    nodes[x].size += nodes[y].size;
}

fn similarity(a: &[u8], p1: &[u8], p2: &[u8]) -> Option<u64> {
    let mut result1 = 0;
    let mut result2 = 0;
    for (i, c) in a.iter().enumerate() {
        if p1[i] != *c && p2[i] != *c {
            return None;
        }
        if p1[i] == *c {
            result1 += 1;
        }
        if p2[i] == *c {
            result2 += 1;
        }
    }
    Some(result1 * result2)
}

fn find_families(members: &mut [FamilyMember]) {
    'child: for child in 0..members.len() {
        for parent1 in 0..members.len() {
            if parent1 == child {
                continue;
            }
            for parent2 in parent1 + 1..members.len() {
                if parent2 == child {
                    continue;
                }
                if let Some(s) = similarity(
                    &members[child].dna,
                    &members[parent1].dna,
                    &members[parent2].dna,
                ) {
                    members[child].similarity = s;
                    union(find(child, members), find(parent1, members), members);
                    union(find(child, members), find(parent2, members), members);
                    union(find(parent1, members), find(parent2, members), members);
                    continue 'child;
                }
            }
        }
    }
}

fn parse(path: &str) -> Vec<FamilyMember> {
    let input = fs::read_to_string(path).expect("Could not read file");
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (id, dna) = l.split_once(':').unwrap();
            FamilyMember {
                id: id.parse().unwrap(),
                parent: i,
                size: 1,
                dna: dna.as_bytes().to_vec(),
                similarity: 0,
            }
        })
        .collect()
}

fn main() {
    // part 1
    let mut members = parse("everybody_codes_e2025_q09_p1.txt");
    find_families(&mut members);
    println!(
        "{}",
        members
            .into_iter()
            .find(|m| m.similarity != 0)
            .unwrap()
            .similarity
    );

    // part 2
    let mut members = parse("everybody_codes_e2025_q09_p2.txt");
    find_families(&mut members);
    println!("{}", members.into_iter().map(|m| m.similarity).sum::<u64>());

    // part 3
    let mut members = parse("everybody_codes_e2025_q09_p3.txt");
    find_families(&mut members);

    let mut max_size = 0;
    for m in &members {
        max_size = max_size.max(m.size);
    }

    let mut total = 0;
    for i in 0..members.len() {
        let parent = find(i, &mut members);
        if members[parent].size == max_size {
            total += members[i].id;
        }
    }

    println!("{total}");
}
