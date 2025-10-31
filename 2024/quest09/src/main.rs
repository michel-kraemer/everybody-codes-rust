use std::fs;

fn ways(n: u64, dots: &[u64], cache: &mut Vec<u64>) -> u64 {
    if n == 0 {
        return 0;
    }

    if cache[n as usize] != u64::MAX {
        return cache[n as usize];
    }

    let mut r = u64::MAX;
    for &d in dots {
        if d == 1 {
            r = r.min(n);
        } else if n >= d {
            r = r.min(ways(n - d, dots, cache) + 1);
        }
    }

    cache[n as usize] = r;

    r
}

fn main() {
    // part 1 + 2
    for part in [1, 2] {
        let input = fs::read_to_string(format!("everybody_codes_e2024_q09_p{}.txt", part)).unwrap();
        let dots = if part == 1 {
            vec![1, 3, 5, 10]
        } else {
            vec![1, 3, 5, 10, 15, 16, 20, 24, 25, 30]
        };
        let mut beetles = 0;
        let mut cache = vec![u64::MAX; 1024];
        for l in input.lines() {
            let brightness = l.parse::<u64>().unwrap();
            if brightness >= cache.len() as u64 {
                cache.resize(brightness as usize + 1, u64::MAX);
            }
            beetles += ways(brightness, &dots, &mut cache);
        }
        println!("{}", beetles);
    }

    // part 3
    let input = fs::read_to_string("everybody_codes_e2024_q09_p3.txt").unwrap();
    let dots = [
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ];
    let mut beetles = 0;
    let mut cache = vec![u64::MAX; 1024];
    for l in input.lines() {
        let brightness = l.parse::<u64>().unwrap();
        let mut min_beetles = u64::MAX;
        for left in brightness.div_ceil(2) - 50..=brightness / 2 {
            let right = brightness - left;
            if left >= cache.len() as u64 {
                cache.resize(left as usize + 1, u64::MAX);
            }
            if right >= cache.len() as u64 {
                cache.resize(right as usize + 1, u64::MAX);
            }
            let b1 = ways(left, &dots, &mut cache);
            let b2 = ways(right, &dots, &mut cache);
            min_beetles = min_beetles.min(b1 + b2);
        }
        beetles += min_beetles;
    }
    println!("{}", beetles);
}
