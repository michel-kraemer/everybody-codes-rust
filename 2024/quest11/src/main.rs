use std::collections::HashMap;
use std::fs;

fn dfs<'a>(
    termite: &'a str,
    days: usize,
    map: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<(&'a str, usize), usize>,
) -> usize {
    if days == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(&(termite, days)) {
        return *cached;
    }

    let mut result = 0;
    for &n in map.get(termite).unwrap() {
        result += dfs(n, days - 1, map, cache);
    }

    cache.insert((termite, days), result);

    result
}

fn main() {
    for part in [1, 2, 3] {
        let input = fs::read_to_string(format!("everybody_codes_e2024_q11_p{}.txt", part)).unwrap();
        let lines = input.lines().collect::<Vec<_>>();

        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        for l in lines {
            let (from, to) = l.split_once(":").unwrap();
            let to = to.split(",").collect::<Vec<_>>();
            map.insert(from, to);
        }

        let days = match part {
            1 => 4,
            2 => 10,
            _ => 20,
        };
        let initial = match part {
            1 => vec!["A"],
            2 => vec!["Z"],
            _ => map.keys().copied().collect::<Vec<_>>(),
        };

        let mut largest = 0usize;
        let mut smallest = usize::MAX;
        let mut cache = HashMap::new();
        for k in initial {
            let bl = dfs(k, days, &map, &mut cache);
            largest = largest.max(bl);
            smallest = smallest.min(bl);
        }
        if part == 3 {
            println!("{}", largest - smallest);
        } else {
            println!("{}", largest);
        }
    }
}
