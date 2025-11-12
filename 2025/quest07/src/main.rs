use std::{collections::HashMap, fs};

fn parse(file: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<char>>) {
    let input = fs::read_to_string(file).expect("Could not read file");

    let (names, rules) = input.split_once("\n\n").unwrap();

    let names = names
        .split(",")
        .map(|n| n.chars().collect())
        .collect::<Vec<_>>();

    let rules = rules
        .lines()
        .map(|r| {
            let (left, right) = r.split_once(" > ").unwrap();
            let right = right
                .split(',')
                .map(|s| {
                    assert_eq!(1, s.len());
                    s.chars().next().unwrap()
                })
                .collect::<Vec<_>>();
            assert_eq!(1, left.len());
            (left.chars().next().unwrap(), right)
        })
        .collect();

    (names, rules)
}

fn check_name(name: &[char], rules: &HashMap<char, Vec<char>>) -> bool {
    for w in name.windows(2) {
        if let Some(e) = rules.get(&w[0])
            && e.contains(&w[1])
        {
            // OK
        } else {
            return false;
        }
    }
    true
}

fn dfs(
    rules: &HashMap<char, Vec<char>>,
    cur: char,
    len: usize,
    cache: &mut HashMap<(char, usize), u64>,
) -> u64 {
    if let Some(c) = cache.get(&(cur, len)) {
        return *c;
    }
    let mut result = if (7..=11).contains(&len) { 1 } else { 0 };
    if len < 11
        && let Some(e) = rules.get(&cur)
    {
        for &next in e {
            result += dfs(rules, next, len + 1, cache);
        }
    }
    cache.insert((cur, len), result);
    result
}

fn main() {
    // part 1
    let (names, rules) = parse("everybody_codes_e2025_q07_p1.txt");
    for name in names {
        if check_name(&name, &rules) {
            println!("{}", name.iter().collect::<String>());
            break;
        }
    }

    // part 2
    let (names, rules) = parse("everybody_codes_e2025_q07_p2.txt");
    let mut total = 0;
    for (i, name) in names.iter().enumerate() {
        if check_name(name, &rules) {
            total += i + 1;
        }
    }
    println!("{total}");

    // part 3
    let mut cache = HashMap::new();
    let (prefixes, rules) = parse("everybody_codes_e2025_q07_p3.txt");
    let mut total = 0;
    for prefix in &prefixes {
        if check_name(prefix, &rules) {
            if prefixes
                .iter()
                .any(|other| prefix != other && prefix.starts_with(other))
            {
                // no need to check this prefix - it's already covered by
                // another, shorter one
                continue;
            }

            total += dfs(
                &rules,
                *prefix.iter().last().unwrap(),
                prefix.len(),
                &mut cache,
            );
        }
    }
    println!("{total}");
}
