use std::fs;

fn parse(path: &str) -> Vec<u64> {
    let input = fs::read_to_string(path).expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn phase1(ducks: &mut [u64], max_rounds: Option<u64>) -> u64 {
    let mut rounds: u64 = 0;
    loop {
        let mut changed = false;

        for i in 0..ducks.len() - 1 {
            if ducks[i] > ducks[i + 1] {
                ducks[i + 1] += 1;
                ducks[i] -= 1;
                changed = true;
            }
        }

        if !changed {
            break;
        }
        rounds += 1;
        if let Some(max_rounds) = max_rounds
            && rounds == max_rounds
        {
            break;
        }
    }
    rounds
}

fn phase2(ducks: &mut [u64], mut rounds: u64, max_rounds: u64) -> u64 {
    loop {
        let mut changed = false;

        for i in 0..ducks.len() - 1 {
            if ducks[i] < ducks[i + 1] {
                ducks[i + 1] -= 1;
                ducks[i] += 1;
                changed = true;
            }
        }

        if !changed {
            break;
        }
        rounds += 1;
        if rounds == max_rounds {
            break;
        }
    }

    rounds
}

fn count_phase2_rounds(ducks: &[u64]) -> u64 {
    let mut result = 0;

    // calculate how many ducks each column will have at the end
    let min = *ducks.iter().min().unwrap();
    let mut over = 0;
    for &d in ducks {
        if d > min {
            over += d - min;
        }
    }
    let target = over / ducks.len() as u64 + min;

    // the number of rounds required to balance the columns is simply the number
    // of missing ducks in the columns that are lower than the target
    for &d in ducks {
        if d < target {
            result += target - d;
        } else {
            break;
        }
    }

    result
}

fn main() {
    // part 1
    let mut ducks = parse("everybody_codes_e2025_q11_p1.txt");
    let rounds = phase1(&mut ducks, Some(10));
    phase2(&mut ducks, rounds, 10);
    println!(
        "{}",
        ducks
            .into_iter()
            .enumerate()
            .map(|(i, d)| (i as u64 + 1) * d)
            .sum::<u64>()
    );

    // part 2
    let mut ducks = parse("everybody_codes_e2025_q11_p2.txt");
    let rounds = phase1(&mut ducks, None);
    println!("{}", rounds + count_phase2_rounds(&ducks));

    // part 3
    let mut ducks = parse("everybody_codes_e2025_q11_p3.txt");
    let rounds = phase1(&mut ducks, None);
    println!("{}", rounds + count_phase2_rounds(&ducks));
}
