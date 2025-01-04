use permutations::permutations_lexicographic;
use std::fs;

mod permutations;

pub const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Action {
    Inc,
    Dec,
    Maintain,
}

fn parse(input: &str) -> Vec<(&str, Vec<Action>)> {
    input
        .lines()
        .map(|l| {
            let (name, actions) = l.split_once(":").unwrap();
            let actions = actions
                .split(",")
                .map(|a| match a {
                    "+" => Action::Inc,
                    "-" => Action::Dec,
                    "=" => Action::Maintain,
                    _ => panic!(),
                })
                .collect::<Vec<_>>();
            (name, actions)
        })
        .collect()
}

fn parse_track(mut plan: Vec<Vec<u8>>) -> Vec<Action> {
    let width = plan[0].len();
    let height = plan.len();

    let mut pos = (1, 0);
    let mut result = Vec::new();
    loop {
        let c = plan[pos.1][pos.0];
        let action = match c {
            b'+' => Action::Inc,
            b'-' => Action::Dec,
            b'=' | b'S' => Action::Maintain,
            _ => panic!(),
        };
        result.push(action);

        if c == b'S' {
            break;
        }

        plan[pos.1][pos.0] = b' ';

        for (dx, dy) in DIRS {
            let nx = (pos.0 as i32) + dx;
            let ny = (pos.1 as i32) + dy;
            if nx >= 0
                && ny >= 0
                && nx < width as i32
                && ny < height as i32
                && plan[ny as usize][nx as usize] != b' '
            {
                pos = (nx as usize, ny as usize);
                break;
            }
        }
    }

    result
}

fn race(actions: &[Action], track: &[Action], rounds: usize) -> u64 {
    let mut power = 10u64;
    let mut score = 0;
    let mut j = 0;
    for _ in 0..rounds {
        for mut action in track {
            if *action == Action::Maintain {
                action = &actions[j % actions.len()];
            }
            match action {
                Action::Inc => power += 1,
                Action::Dec => power = power.saturating_sub(1),
                Action::Maintain => {}
            }
            score += power;
            j += 1;
        }
    }

    score
}

fn main() {
    // part 1
    let input1 = fs::read_to_string("everybody_codes_e2024_q07_p1.txt").unwrap();
    let charriots1 = parse(&input1);
    let mut scores1 = charriots1
        .into_iter()
        .map(|(name, actions)| (name, race(&actions, &[Action::Maintain], 10)))
        .collect::<Vec<_>>();
    scores1.sort_unstable_by_key(|s| s.1);
    println!(
        "{}",
        scores1.into_iter().rev().map(|s| s.0).collect::<String>()
    );

    // part 2
    let input2 = fs::read_to_string("everybody_codes_e2024_q07_p2.txt").unwrap();
    let charriots2 = parse(&input2);
    let track2 = fs::read_to_string("track_p2.txt")
        .unwrap()
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let track2 = parse_track(track2);
    let mut scores2 = charriots2
        .into_iter()
        .map(|(name, actions)| (name, race(&actions, &track2, 10)))
        .collect::<Vec<_>>();
    scores2.sort_unstable_by_key(|s| s.1);
    println!(
        "{}",
        scores2.into_iter().rev().map(|s| s.0).collect::<String>()
    );

    // part 3
    let input3 = fs::read_to_string("everybody_codes_e2024_q07_p3.txt").unwrap();
    let charriots3 = parse(&input3);
    let track3 = fs::read_to_string("track_p3.txt")
        .unwrap()
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let track3 = parse_track(track3);

    // The race doesn't need to take 2024 rounds. Since each plan is 11 actions
    // long and the power apparently never drops to zero, the amount of essence
    // we can gather will repeat after 11 rounds. Further, since 2024 is
    // divisible by 11, the winner will already have been decided at that point.
    let rounds3 = 11; // instead of 2024

    let score_a = race(&charriots3.into_iter().next().unwrap().1, &track3, rounds3);

    let template = vec![
        Action::Inc,
        Action::Inc,
        Action::Inc,
        Action::Inc,
        Action::Inc,
        Action::Dec,
        Action::Dec,
        Action::Dec,
        Action::Maintain,
        Action::Maintain,
        Action::Maintain,
    ];
    let mut total = 0;
    for t in permutations_lexicographic(&template) {
        let score_b = race(&t, &track3, rounds3);
        if score_b > score_a {
            total += 1;
        }
    }
    println!("{}", total);
}
