use std::fs;

fn parse(file: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let input = fs::read_to_string(file).expect("Could not read file");

    let (names, rules) = input.split_once("\n\n").unwrap();

    let names = names
        .split(",")
        .map(|n| n.chars().collect())
        .collect::<Vec<_>>();

    let mut parsed_rules = vec![Vec::new(); 128];
    for r in rules.lines() {
        let (left, right) = r.split_once(" > ").unwrap();
        let right = right
            .split(',')
            .map(|s| {
                assert_eq!(1, s.len());
                s.chars().next().unwrap()
            })
            .collect::<Vec<_>>();
        assert_eq!(1, left.len());
        parsed_rules[left.chars().next().unwrap() as usize] = right;
    }

    (names, parsed_rules)
}

fn check_name(name: &[char], rules: &[Vec<char>]) -> bool {
    for w in name.windows(2) {
        if rules[w[0] as usize].contains(&w[1]) {
            // OK
        } else {
            return false;
        }
    }
    true
}

fn dfs(rules: &[Vec<char>], cur: char, len: usize, cache: &mut [u64]) -> u64 {
    let cidx = (cur as u8 - b'a') as usize * 11 + (len - 1);
    if cache[cidx] != u64::MAX {
        return cache[cidx];
    }
    let mut result = if (7..=11).contains(&len) { 1 } else { 0 };
    if len < 11 {
        for &next in &rules[cur as usize] {
            result += dfs(rules, next, len + 1, cache);
        }
    }
    cache[cidx] = result;
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
    let mut cache = vec![u64::MAX; 26 * 11];
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
