use std::{cmp::Reverse, collections::HashMap, fs};

struct Entry {
    id: u64,
    quest: u64,
    part: u64,
    time: u64,
}

fn parse(path: &str) -> Vec<Entry> {
    let input = fs::read_to_string(path).expect("Could not read file");
    let mut result = Vec::new();
    for l in input.lines() {
        let s = l.split_ascii_whitespace().collect::<Vec<_>>();
        let (_, id) = s[0].split_once('=').unwrap();
        let (_, quest) = s[1].split_once('=').unwrap();
        let (_, part) = s[2].split_once('=').unwrap();
        let (_, time) = s[3].split_once('=').unwrap();
        result.push(Entry {
            id: id.parse().unwrap(),
            quest: quest.parse().unwrap(),
            part: part.parse().unwrap(),
            time: time.parse().unwrap(),
        });
    }
    result
}

fn checksum(participants: &[u64]) -> u64 {
    let mut checksum = 0;
    let mut last_value = participants[0];
    let mut last_place = 1;
    for (i, &value) in participants.iter().enumerate() {
        if value != last_value {
            last_value = value;
            last_place = i as u64 + 1;
        }
        checksum += last_place * value;
    }
    checksum
}

fn main() {
    // part 1
    let entries = parse("everybody_codes_e2026_q04_p1.txt");

    let mut participants: HashMap<u64, u64> = HashMap::new();
    for e in entries {
        *participants.entry(e.id).or_default() += e.time;
    }

    let mut participants = participants.values().copied().collect::<Vec<_>>();
    participants.sort();

    let total = checksum(&participants);
    println!("{total}");

    // part 2
    let entries = parse("everybody_codes_e2026_q04_p2.txt");

    let mut participants: HashMap<u64, (Reverse<u64>, u64)> = HashMap::new();
    for e in entries {
        participants.entry(e.id).or_default().0.0 += 1;
        participants.entry(e.id).or_default().1 += e.time;
    }

    let mut participants = participants.values().copied().collect::<Vec<_>>();
    participants.sort();
    let participants = participants.into_iter().map(|p| p.1).collect::<Vec<_>>();

    let total = checksum(&participants);
    println!("{total}");

    // part 3
    let mut entries = parse("everybody_codes_e2026_q04_p3.txt");

    // make sure entries are sorted by part
    entries.sort_by_key(|e| e.part);

    // accumulate times
    let mut quests: HashMap<(u64, u64), HashMap<u64, u64>> = HashMap::new();
    for e in entries {
        let prev_time = quests
            .get(&(e.quest, e.part - 1))
            .and_then(|m| m.get(&e.id))
            .copied()
            .unwrap_or_default();
        quests
            .entry((e.quest, e.part))
            .or_default()
            .insert(e.id, prev_time + e.time);
    }

    // score participants per quest and part
    let mut participants: HashMap<u64, u64> = HashMap::new();
    for ((_quest, part), times) in quests {
        let max_points = match part {
            1 => 50,
            2 => 100,
            _ => 150,
        };

        let mut times = times.into_iter().collect::<Vec<_>>();
        times.sort_by_key(|p| p.1);

        let mut points = max_points;
        let mut last_time = times[0].1;
        let mut last_points = max_points;
        for t in times {
            if t.1 != last_time {
                last_time = t.1;
                last_points = points;
            }
            *participants.entry(t.0).or_default() += last_points;
            points = points.saturating_sub(1);
        }
    }

    // sort all scores
    let mut participants = participants.values().copied().collect::<Vec<_>>();
    participants.sort();
    participants.reverse();

    let total = checksum(&participants);
    println!("{total}");
}
