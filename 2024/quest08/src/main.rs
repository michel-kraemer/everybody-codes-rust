use std::fs;

fn part1() {
    let available_blocks = fs::read_to_string("everybody_codes_e2024_q08_p1.txt")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let mut required_blocks = 0;
    let mut width = 1;
    while required_blocks < available_blocks {
        required_blocks += width;
        if required_blocks >= available_blocks {
            break;
        }
        width += 2;
    }

    println!("{}", width * (required_blocks - available_blocks));
}

fn part2() {
    let priests = fs::read_to_string("everybody_codes_e2024_q08_p2.txt")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let acolytes = 1111;
    let available_blocks = 20240000;

    let mut required_blocks = 1;
    let mut width = 1;
    let mut height = 1;
    while required_blocks < available_blocks {
        height = (height * priests) % acolytes;
        width += 2;
        required_blocks += width * height;
    }

    println!("{}", width * (required_blocks - available_blocks));
}

fn part3(available_blocks: u64) {
    let priests = fs::read_to_string("everybody_codes_e2024_q08_p3.txt")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let acolytes = 10;

    let mut required_blocks = 1;
    let mut width = 1;
    let mut height = 1;
    let mut heights = vec![1];
    while required_blocks < available_blocks {
        height = (height * priests) % acolytes + acolytes;
        width += 2;
        required_blocks += width * height;
        heights.push(height);
    }

    let mut total_height = heights.iter().sum::<u64>();
    let mut empty = 0;
    for (i, h) in heights.iter().take(heights.len() - 1).enumerate() {
        let e = (priests * width * total_height) % acolytes;
        empty += if i == 0 { e } else { e * 2 };
        total_height -= h;
    }

    println!("{}", required_blocks - empty - available_blocks);
}

fn main() {
    part1();
    part2();
    part3(202400000u64); // new number of available blocks
    part3(202400000000u64); // original number before the puzzle was updated
}
