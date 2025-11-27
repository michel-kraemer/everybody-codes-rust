use std::fs;

struct Branch {
    thickness: i64,
    source: usize,
}

enum Plant {
    Free {
        energy: i64,
    },
    Inner {
        thickness: i64,
        branches: Vec<Branch>,
    },
}

fn parse(path: &str) -> (Vec<Plant>, Vec<Vec<i64>>) {
    let input = fs::read_to_string(path).expect("Could not read file");
    let (left, right) = input.trim().split_once("\n\n\n").unwrap_or((&input, ""));

    // parse plants
    let blocks = left.split("\n\n").collect::<Vec<_>>();
    let mut plants = Vec::new();
    for b in blocks {
        let lines = b.lines().collect::<Vec<_>>();
        let thickness = lines[0][0..lines[0].len() - 1]
            .split(' ')
            .collect::<Vec<_>>()[4]
            .parse::<i64>()
            .unwrap();
        if lines[1].starts_with("- free") {
            plants.push(Plant::Free { energy: 1 });
        } else {
            let mut branches = Vec::new();
            for l in &lines[1..] {
                let source = l.split(' ').collect::<Vec<_>>()[4].parse::<i64>().unwrap();
                let branch_thickness = l.split(' ').collect::<Vec<_>>()[7].parse::<i64>().unwrap();
                branches.push(Branch {
                    thickness: branch_thickness,
                    source: (source - 1) as usize,
                });
            }
            plants.push(Plant::Inner {
                thickness,
                branches,
            });
        }
    }

    // parse test cases
    let test_cases = right
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (plants, test_cases)
}

fn calculate_energy(i: usize, plants: &[Plant]) -> i64 {
    match &plants[i] {
        Plant::Free { energy } => *energy,
        Plant::Inner {
            thickness,
            branches,
        } => {
            let mut result = 0;
            for b in branches {
                result += calculate_energy(b.source, plants) * b.thickness;
            }
            if *thickness > result { 0 } else { result }
        }
    }
}

fn calculate_energy_with_input(i: usize, plants: &mut [Plant], input: &[i64]) -> i64 {
    for (i, e) in input.iter().enumerate() {
        let Plant::Free { ref mut energy } = plants[i] else {
            panic!("Expected free plant at position {i}");
        };
        *energy = *e;
    }
    calculate_energy(i, plants)
}

fn main() {
    // part 1
    let (plants, _) = parse("everybody_codes_e2025_q18_p1.txt");
    println!("{}", calculate_energy(plants.len() - 1, &plants));

    // part 2
    let (mut plants, test_cases) = parse("everybody_codes_e2025_q18_p2.txt");
    let mut total = 0;
    for case in test_cases {
        total += calculate_energy_with_input(plants.len() - 1, &mut plants, &case);
    }
    println!("{total}");

    // part 3 ...
    let (mut plants, test_cases) = parse("everybody_codes_e2025_q18_p3.txt");

    // count free plants
    let n_free_plants = plants
        .iter()
        .filter(|p| matches!(p, Plant::Free { .. }))
        .count();

    // find all plants that only depend on free plants
    let second_level_plants = plants
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            let Plant::Inner { branches, .. } = p else {
                return false;
            };
            branches
                .iter()
                .all(|b| matches!(plants[b.source], Plant::Free { .. }))
        })
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    // try to maximize the energy of all second-level plants
    let mut maximum_test_case: Vec<Option<i64>> = vec![None; n_free_plants];
    for &i in &second_level_plants {
        let Plant::Inner { branches, .. } = &plants[i] else {
            unreachable!("Expected inner plant at position {i}");
        };
        for b in branches {
            let value = if b.thickness > 0 { 1 } else { 0 };
            if let Some(old) = maximum_test_case[b.source]
                && old != value
            {
                panic!(
                    "Expected free plant at position {} to have energy {value}",
                    b.source
                );
            }
            maximum_test_case[b.source] = Some(value);
        }
    }
    let maximum_test_case = maximum_test_case
        .into_iter()
        .map(|v| v.expect("Expecting all values to be defined"))
        .collect::<Vec<_>>();

    // calculate maximum energy
    let max_energy = calculate_energy_with_input(plants.len() - 1, &mut plants, &maximum_test_case);

    // sum up differences
    let mut total = 0;
    for case in test_cases {
        let e = calculate_energy_with_input(plants.len() - 1, &mut plants, &case);
        if e > 0 {
            total += max_energy - e;
        }
    }
    println!("{total}");
}
