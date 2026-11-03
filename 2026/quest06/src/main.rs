use std::collections::{HashMap, HashSet};
use std::fs;

fn parse(path: &str) -> HashMap<String, Vec<String>> {
    let input = fs::read_to_string(path).expect("Could not read file");
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    for l in input.lines() {
        let (to, from) = l.split_once(':').unwrap();
        let from = from.split(',').map(|f| f.to_string()).collect::<Vec<_>>();
        for f in &from {
            deps.entry(f.to_string()).or_default();
        }
        deps.insert(to.to_string(), from);
    }
    deps
}

fn get_incredients_transitive(
    to: &str,
    deps: &HashMap<String, Vec<String>>,
    result: &mut HashSet<String>,
) {
    if let Some(from) = deps.get(to) {
        for f in from {
            if result.insert(f.to_string()) {
                get_incredients_transitive(f, deps, result);
            }
        }
    }
}

fn main() {
    // part 1
    let deps = parse("everybody_codes_e2026_q06_p1.txt");
    let mut incredients = HashSet::new();
    get_incredients_transitive("NULL", &deps, &mut incredients);
    incredients.remove("NULL");
    println!("{}", incredients.len());

    // part 2
    let deps = parse("everybody_codes_e2026_q06_p2.txt");
    let mut total = 0;
    for d in deps.keys() {
        if **d == *"NULL" {
            continue;
        }
        let mut incredients = HashSet::new();
        get_incredients_transitive(d, &deps, &mut incredients);
        if incredients.contains("NULL") {
            total += 1;
        }
    }
    println!("{total}");

    // part 3
    let deps = parse("everybody_codes_e2026_q06_p3.txt");
    let mut total = 0;
    for d in deps.keys() {
        if **d == *"NULL" {
            continue;
        }
        let mut incredients = HashSet::new();
        get_incredients_transitive(d, &deps, &mut incredients);
        if incredients.iter().filter(|i| i.starts_with("NULL")).count() >= 2 {
            total += 1;
        }
    }
    println!("{total}");
}
