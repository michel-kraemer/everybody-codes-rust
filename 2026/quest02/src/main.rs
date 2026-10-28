use std::collections::HashMap;
use std::fs;

fn main() {
    for part in 1..=3 {
        let input = fs::read_to_string(format!("everybody_codes_e2026_q02_p{part}.txt"))
            .expect("Could not read file");

        let mut total = 0;
        for l in input.lines() {
            // parse input into graph - count incoming edges and record outgoing
            // edges
            let pairs = l.split_ascii_whitespace().collect::<Vec<_>>();
            let mut incoming: HashMap<u8, usize> = HashMap::new();
            let mut outgoing: HashMap<u8, Vec<u8>> = HashMap::new();
            for p in pairs {
                let mut digit1 = p.as_bytes()[0];
                let mut digit2 = p.as_bytes()[2];
                if p.as_bytes()[1] == b'<' {
                    (digit1, digit2) = (digit2, digit1);
                }
                *incoming.entry(digit1).or_default() += 1;
                incoming.entry(digit2).or_insert(0);
                outgoing.entry(digit2).or_default().push(digit1);
            }

            // find nodes without incoming edges and make sure there's only one
            let mut s = incoming
                .iter()
                .filter(|&(_, i)| *i == 0)
                .map(|(n, _)| *n)
                .collect::<Vec<_>>();
            assert_eq!(
                s.len(),
                1,
                "Problem not solvable. There is more than one node without incoming edges!"
            );

            // perform topological sort
            let mut l = Vec::new();
            while let Some(n) = s.pop() {
                l.push(n);
                if let Some(neighbors) = outgoing.get(&n) {
                    for e in neighbors {
                        let entry = incoming.get_mut(e).unwrap();
                        *entry -= 1;
                        if *entry == 0 {
                            s.push(*e);
                        }
                    }
                }
            }

            // l contains sorted digits in reverse order. Convert them to a
            // number.
            let mut r = 0;
            for d in l.iter().rev() {
                r *= 10;
                r += (d - b'0') as u64;
            }

            total += r;
        }

        println!("{total}");
    }
}
