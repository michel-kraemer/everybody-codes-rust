use std::collections::HashSet;
use std::fs;

fn main() {
    // part 1
    let input =
        fs::read_to_string("everybody_codes_e2024_q02_p1.txt").expect("Could not read file");

    let (words, text) = input.split_once("\n\n").unwrap();
    let words = words[6..]
        .split(",")
        .map(|w| w.to_string())
        .collect::<Vec<_>>();

    let mut total = 0;
    for i in 0..text.len() {
        for w in &words {
            if text[i..].starts_with(w) {
                total += 1;
            }
        }
    }
    println!("{}", total);

    // part 2
    let input =
        fs::read_to_string("everybody_codes_e2024_q02_p2.txt").expect("Could not read file");

    let (words, text) = input.split_once("\n\n").unwrap();
    let mut words = words[6..]
        .split(",")
        .map(|w| w.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let lines = text
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let reversed_words = words
        .iter()
        .map(|w| w.iter().rev().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    words.extend(reversed_words);

    let mut seen = HashSet::new();
    for (y, l) in lines.iter().enumerate() {
        for x in 0..l.len() {
            for w in &words {
                let mut found = true;
                for i in 0..w.len() {
                    if x + i == l.len() || l[x + i] != w[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    for k in 0..w.len() {
                        seen.insert((x + k, y));
                    }
                }
            }
        }
    }
    println!("{}", seen.len());

    // part 3
    let input =
        fs::read_to_string("everybody_codes_e2024_q02_p3.txt").expect("Could not read file");

    let (words, text) = input.split_once("\n\n").unwrap();
    let mut words = words[6..]
        .split(",")
        .map(|w| w.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let lines = text
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let reversed_words = words
        .iter()
        .map(|w| w.iter().rev().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    words.extend(reversed_words);

    let mut seen = HashSet::new();
    for (y, l) in lines.iter().enumerate() {
        for x in 0..l.len() {
            for w in &words {
                let mut found = true;
                for i in 0..w.len() {
                    if l[(x + i) % l.len()] != w[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    for i in 0..w.len() {
                        seen.insert(((x + i) % l.len(), y));
                    }
                }
            }
        }
    }
    for x in 0..lines[0].len() {
        for y in 0..lines.len() {
            for w in &words {
                let mut found = true;
                for i in 0..w.len() {
                    if y + i == lines.len() || lines[y + i][x] != w[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    for i in 0..w.len() {
                        seen.insert((x, y + i));
                    }
                }
            }
        }
    }
    println!("{}", seen.len());
}
