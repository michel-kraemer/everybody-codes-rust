use std::fs;

fn main() {
    for part in [1, 2, 3] {
        let input = fs::read_to_string(format!("everybody_codes_e2025_q01_p{}.txt", part))
            .expect("Could not read file");

        let (names, instructions) = input.trim().split_once("\n\n").unwrap();
        let mut names = names.split(",").collect::<Vec<_>>();
        let instructions = instructions.split(",").collect::<Vec<_>>();

        let mut pos: i32 = 0;
        for i in instructions {
            let (dir, steps) = i.split_at(1);
            let steps = steps.parse::<i32>().unwrap();
            match dir {
                "L" => pos -= steps,
                "R" => pos += steps,
                _ => unreachable!(),
            }

            match part {
                1 => {
                    pos = pos.clamp(0, names.len() as i32 - 1);
                }
                2 => {
                    pos = pos.rem_euclid(names.len() as i32);
                }
                3 => {
                    pos = pos.rem_euclid(names.len() as i32);
                    names.swap(0, pos as usize);
                    pos = 0;
                }
                _ => unreachable!(),
            }
        }

        println!("{}", names[pos as usize]);
    }
}
