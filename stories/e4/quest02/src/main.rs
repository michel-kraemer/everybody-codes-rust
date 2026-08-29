use std::collections::HashSet;
use std::fs;

#[derive(Default)]
struct Configuration {
    start: (i64, i64),
    a: (i64, i64),
    b: (i64, i64),
    c: (i64, i64),
    moves: Vec<(i64, i64)>,
}

fn parse(file: &str) -> Configuration {
    let input = fs::read_to_string(file).expect("Could not read file");

    let mut result = Configuration::default();
    let mut moves = None;
    for l in input.lines() {
        let (key, value) = l.split_once('=').unwrap();
        if key == "MOVES" {
            moves = Some(value.to_string());
        } else {
            let (x, y) = value[1..value.len() - 1].split_once(',').unwrap();
            let x = x.parse::<i64>().unwrap();
            let y = y.parse::<i64>().unwrap();
            match key {
                "START" => result.start = (x, y),
                "A" => result.a = (x, y),
                "B" => result.b = (x, y),
                "C" => result.c = (x, y),
                _ => unreachable!("Invalid key: {key}"),
            }
        }
    }

    if let Some(moves) = moves {
        result.moves = moves
            .chars()
            .map(|m| match m {
                'A' => result.a,
                'B' => result.b,
                'C' => result.c,
                _ => unreachable!("Invalid destination: {m}"),
            })
            .collect::<Vec<_>>();
    }

    result
}

fn dfs<F>(
    pos: (i64, i64),
    next_dests: &mut F,
    illuminated: &mut HashSet<(i64, i64)>,
    stop_at_visited: bool,
) where
    F: FnMut() -> Vec<(i64, i64)>,
{
    if stop_at_visited && illuminated.contains(&pos) {
        return;
    }
    illuminated.insert(pos);

    for dest in next_dests() {
        let nx = (pos.0 + dest.0) / 2;
        let ny = (pos.1 + dest.1) / 2;
        dfs((nx, ny), next_dests, illuminated, stop_at_visited);
    }
}

fn flies(illuminated: &HashSet<(i64, i64)>) -> usize {
    let mut result: HashSet<(i64, i64)> = HashSet::new();

    for i in illuminated {
        for d in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let nx = i.0 + d.0;
            let ny = i.1 + d.1;
            if !illuminated.contains(&(nx, ny)) {
                result.insert((nx, ny));
            }
        }
    }

    result.len()
}

fn main() {
    // part 1
    let config = parse("everybody_codes_e4_q02_p1.txt");
    let mut illuminated = HashSet::new();
    let mut mi = config.moves.iter();
    dfs(
        config.start,
        &mut || mi.next().map(|dest| vec![*dest]).unwrap_or_default(),
        &mut illuminated,
        false,
    );
    println!("{}", illuminated.len());

    // part 2
    let config = parse("everybody_codes_e4_q02_p2.txt");
    let mut illuminated = HashSet::new();
    let mut mi = config.moves.iter();
    dfs(
        config.start,
        &mut || mi.next().map(|dest| vec![*dest]).unwrap_or_default(),
        &mut illuminated,
        false,
    );
    println!("{}", flies(&illuminated));

    // part 3
    let config = parse("everybody_codes_e4_q02_p3.txt");
    let mut illuminated = HashSet::new();
    dfs(
        config.start,
        &mut || vec![config.a, config.b, config.c],
        &mut illuminated,
        true,
    );
    println!("{}", flies(&illuminated));
}
