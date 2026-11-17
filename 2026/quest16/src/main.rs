use std::fs;

fn sort(mut input: Vec<u8>) -> (usize, usize) {
    let mut total_length = 0;
    let mut swaps = 0;

    // we only need to sort the first three letters, the fourth letter will be
    // left already sorted at the end
    for _ in 0..3 {
        let current = input[0];
        let mut left = 0;
        let mut right = input.len() - 1;
        let mut new_input = Vec::new();

        while left < right {
            // skip `current` at left side
            while left < right && input[left] == current {
                left += 1;
            }

            // find last `current` at right side
            while right > left && input[right] != current {
                new_input.push(input[right]);
                right -= 1;
            }

            // count swap
            if right > left {
                total_length += right - left + 1;
                swaps += 1;
            }

            // skip `current` at right side
            while right > left && input[right] == current {
                right -= 1;
            }

            // find next `current` at left side
            while left <= right && input[left] != current {
                new_input.push(input[left]);
                left += 1;
            }

            // count swap
            if right > left {
                total_length += right - left + 1;
                swaps += 1;
            }
        }

        new_input.reverse();
        input = new_input;
    }

    (total_length, swaps)
}

fn main() {
    // part 1
    let input =
        fs::read_to_string("everybody_codes_e2026_q16_p1.txt").expect("Could not read file");
    let bytes = input.trim().bytes().collect::<Vec<_>>();
    let (_, swaps) = sort(bytes);
    println!("{swaps}");

    // part 2
    let input =
        fs::read_to_string("everybody_codes_e2026_q16_p2.txt").expect("Could not read file");
    let bytes = input.trim().bytes().collect::<Vec<_>>().repeat(10);
    let (total, _) = sort(bytes);
    println!("{total}");

    // part 3
    let input =
        fs::read_to_string("everybody_codes_e2026_q16_p3.txt").expect("Could not read file");
    let bytes = input.trim().bytes().collect::<Vec<_>>().repeat(100);
    let (total, _) = sort(bytes);
    println!("{total}");
}
