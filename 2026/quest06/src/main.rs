use std::collections::{HashMap, HashSet};
use std::fs;

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut deps: HashMap<&str, Vec<&str>> = HashMap::new();
    for l in input.lines() {
        let (to, from) = l.split_once(':').unwrap();
        let from = from.split(',').collect::<Vec<_>>();
        deps.insert(to, from);
    }
    deps
}

fn reverse_deps<'a>(deps: HashMap<&'a str, Vec<&'a str>>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut result: HashMap<&str, Vec<&str>> = HashMap::new();
    for (k, v) in deps {
        for f in v {
            result.entry(f).or_default().push(k);
        }
    }
    result
}

fn get_incredients_transitive<'a>(
    to: &str,
    deps: &HashMap<&'a str, Vec<&'a str>>,
    result: &mut HashSet<&'a str>,
) {
    if let Some(from) = deps.get(to) {
        for f in from {
            if result.insert(f) {
                get_incredients_transitive(f, deps, result);
            }
        }
    }
}

fn main() {
    // part 1
    let input =
        fs::read_to_string("everybody_codes_e2026_q06_p1.txt").expect("Could not read file");
    let deps = parse(&input);
    let mut incredients = HashSet::new();
    get_incredients_transitive("NULL", &deps, &mut incredients);
    incredients.remove("NULL");
    println!("{}", incredients.len());

    // part 2
    let input =
        fs::read_to_string("everybody_codes_e2026_q06_p2.txt").expect("Could not read file");
    let deps = reverse_deps(parse(&input));
    let mut incredients = HashSet::new();
    get_incredients_transitive("NULL", &deps, &mut incredients);
    incredients.remove("NULL");
    println!("{}", incredients.len());

    // part 3
    let input =
        fs::read_to_string("everybody_codes_e2026_q06_p3.txt").expect("Could not read file");
    let deps = reverse_deps(parse(&input));
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for d in deps.keys() {
        if d.starts_with("NULL") {
            let mut incredients = HashSet::new();
            get_incredients_transitive(d, &deps, &mut incredients);
            incredients.remove("NULL");
            for i in incredients {
                *counts.entry(i).or_default() += 1;
            }
        }
    }
    println!("{}", counts.into_iter().filter(|c| c.1 >= 2).count());
}
