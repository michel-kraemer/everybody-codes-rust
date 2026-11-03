use std::collections::{HashMap, HashSet};
use std::fs;

fn parse(path: &str) -> HashMap<String, Vec<String>> {
    let input = fs::read_to_string(path).expect("Could not read file");
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    for l in input.lines() {
        let (to, from) = l.split_once(':').unwrap();
        let from = from.split(',').map(|f| f.to_string()).collect::<Vec<_>>();
        deps.insert(to.to_string(), from);
    }
    deps
}

fn reverse_deps(deps: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for (k, v) in deps {
        for f in v {
            result.entry(f.clone()).or_default().push(k.clone());
        }
    }
    result
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
    let deps = reverse_deps(parse("everybody_codes_e2026_q06_p2.txt"));
    let mut incredients = HashSet::new();
    get_incredients_transitive("NULL", &deps, &mut incredients);
    incredients.remove("NULL");
    println!("{}", incredients.len());

    // part 3
    let deps = reverse_deps(parse("everybody_codes_e2026_q06_p3.txt"));
    let mut counts: HashMap<String, usize> = HashMap::new();
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
