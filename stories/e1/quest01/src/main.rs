use std::collections::HashMap;
use std::fs;

fn parse(l: &str) -> (u64, u64, u64, u64, u64, u64, u64) {
    let v = l
        .split_whitespace()
        .map(|i| i.split_once("=").unwrap().1.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let a = v[0];
    let b = v[1];
    let c = v[2];
    let x = v[3];
    let y = v[4];
    let z = v[5];
    let m = v[6];
    (a, b, c, x, y, z, m)
}

fn eni_part1(n: u64, e: u64, m: u64) -> u64 {
    let mut f = 1;
    let mut r = 0;
    let mut p = 1;
    for _ in 0..e {
        p = (p * n) % m;
        r += f * p;
        let digits = if p == 0 { 1 } else { p.ilog10() + 1 };
        for _ in 0..digits {
            f *= 10;
        }
    }
    r
}

fn pow_mod(n: u64, mut b: u64, m: u64) -> u64 {
    let mut a = n;
    let mut result = 1;
    while b > 0 {
        if b & 1 != 0 {
            result *= a;
            result %= m;
        }
        a *= a;
        a %= m;
        b >>= 1;
    }
    result
}

fn eni_part2(n: u64, e: u64, m: u64) -> u64 {
    let mut p = pow_mod(n, e - 5, m);
    let mut f = 1;
    let mut r = 0;
    for _ in 0..5 {
        p = (p * n) % m;
        r += f * p;
        let digits = if p == 0 { 1 } else { p.ilog10() + 1 };
        for _ in 0..digits {
            f *= 10;
        }
    }
    r
}

fn eni_part3(n: u64, e: u64, m: u64) -> u64 {
    let mut seen: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut p = 1;
    let mut sum = 0;
    let mut i = 0;
    while i < e {
        if let Some((j, old_sum)) = seen.get(&p) {
            let cycle_len = i - j;
            let s = sum - old_sum;
            let times = (e - i) / cycle_len;
            sum += s * times;
            i += cycle_len * times;
            seen.clear();
        }

        let prev_p = p;
        seen.insert(prev_p, (i, sum));
        p = (p * n) % m;
        sum += p;
        i += 1;
    }
    sum
}

fn eni(n: u64, e: u64, m: u64, part: usize) -> u64 {
    match part {
        1 => eni_part1(n, e, m),
        2 => eni_part2(n, e, m),
        3 => eni_part3(n, e, m),
        _ => unreachable!(),
    }
}

fn main() {
    for part in [1, 2, 3] {
        let input = fs::read_to_string(format!("everybody_codes_e1_q01_p{}.txt", part))
            .expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();
        let mut max = 0;
        for l in lines {
            let (a, b, c, x, y, z, m) = parse(l);
            let r = eni(a, x, m, part) + eni(b, y, m, part) + eni(c, z, m, part);
            max = max.max(r);
        }
        println!("{}", max);
    }
}
