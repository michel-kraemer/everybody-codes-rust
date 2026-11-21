use std::fs;

/// Count the number of segments in the given substring. The substring is
/// supposed to only consist of base 11 characters (no whitespace and no
/// operators or equals sign)
fn count_segments(s: &[u8]) -> u64 {
    let mut result = 0;
    for b in s {
        result += match b {
            b'0' | b'6' | b'9' | b'A' => 6,
            b'1' => 2,
            b'2' | b'3' | b'5' => 5,
            b'4' => 4,
            b'7' => 3,
            b'8' => 7,
            _ => unreachable!(),
        };
    }
    result
}

/// Split the given equations by ASCII whitespace
fn split(equation: &[u8]) -> Vec<&[u8]> {
    let mut start = 0;
    let mut end = start;
    let mut result = Vec::new();
    while end < equation.len() {
        if equation[end].is_ascii_whitespace() {
            result.push(&equation[start..end]);
            start = end + 1;
        }
        end += 1;
    }
    result.push(&equation[start..end]);
    result
}

/// Parse the given base 11 encoded string
fn parse_radix11(s: &[u8]) -> i64 {
    let mut result = 0;
    for b in s {
        result *= 11;
        result += (*b as char).to_digit(11).unwrap();
    }
    result as i64
}

/// Encode a number into base 11
fn to_radix11(mut i: u64) -> String {
    if i == 0 {
        return "0".to_string();
    }
    let mut result = Vec::new();
    while i > 0 {
        let r = i % 11;
        i /= 11;
        if r == 10 {
            result.push('A');
        } else {
            result.push((b'0' + r as u8) as char);
        }
    }
    result.into_iter().rev().collect::<String>()
}

/// Try to parse the given equation and return the number of segments if
/// successful
fn parse_equation(equation: &[u8]) -> Option<u64> {
    let parts = split(equation);
    let a = parse_radix11(parts[0]);
    let b = parse_radix11(parts[2]);
    let c = parse_radix11(parts[4]);
    let d = parse_radix11(parts[6]);
    let op1 = parts[1][0] == b'+';
    let op2 = parts[5][0] == b'+';

    let left = if op1 { a + b } else { a - b };
    let right = if op2 { c + d } else { c - d };

    if left != right {
        // equation is not correct
        return None;
    }

    // Equation is correct. Count segments.
    let mut total = count_segments(parts[0]);
    total += count_segments(parts[2]);
    total += count_segments(parts[4]);
    total += count_segments(parts[6]);

    if op1 {
        total += 2;
    } else {
        total += 1;
    }

    if op2 {
        total += 2;
    } else {
        total += 1;
    }

    // '=' sign
    total += 2;

    Some(total)
}

/// Add a segment to a character and return a list of possible replacement
/// characters. Return `None` if there are not valid replacement characters.
fn add(b: u8) -> Option<&'static str> {
    match b {
        b'2' | b'4' | b'7' | b'8' | b'+' | b'=' => None,
        b'0' => Some("8"),
        b'1' => Some("7"),
        b'3' => Some("9"),
        b'5' => Some("69"),
        b'6' => Some("8"),
        b'9' => Some("8"),
        b'A' => Some("8"),
        b'-' => Some("+"),
        _ => unreachable!(),
    }
}

/// Remove a segment from a character and return a list of possible replacement
/// characters. Return `None` if there are not valid replacement characters.
fn remove(b: u8) -> Option<&'static str> {
    match b {
        b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'A' | b'-' | b'=' => None,
        b'6' => Some("5"),
        b'7' => Some("1"),
        b'8' => Some("069A"),
        b'9' => Some("35"),
        b'+' => Some("-"),
        _ => unreachable!(),
    }
}

/// Move a segment within a character and return a list of possible replacement
/// characters. Return `None` if there are not valid replacement characters.
fn move_within(b: u8) -> Option<&'static str> {
    match b {
        b'1' | b'4' | b'7' | b'8' | b'+' | b'-' | b'=' => None,
        b'0' => Some("69A"),
        b'2' => Some("3"),
        b'3' => Some("25"),
        b'5' => Some("3"),
        b'6' => Some("09A"),
        b'9' => Some("06A"),
        b'A' => Some("069"),
        _ => unreachable!(),
    }
}

/// Recursively try to move a single segment within the given broken equation
/// and return the number of segments if the equation can be fixed
fn recurse(bytes: &mut [u8], i: usize, has_removed: bool, has_added: bool) -> Option<u64> {
    if i == bytes.len() {
        // unable to fix equation
        return None;
    }

    // skip whitespace characters
    if bytes[i].is_ascii_whitespace() {
        return recurse(bytes, i + 1, has_removed, has_added);
    }

    // try to add a segment if we haven't added one yet
    if !has_added && let Some(replacements) = add(bytes[i]) {
        let ci = bytes[i];
        for &r in replacements.as_bytes() {
            bytes[i] = r;
            if has_removed {
                // We've added and removed a segment. Try to parse the equation.
                let pr = parse_equation(bytes);
                if pr.is_some() {
                    return pr;
                }
            } else {
                // continue with the next character but set the `has_added` flag
                // to `true`
                let pr = recurse(bytes, i + 1, false, true);
                if pr.is_some() {
                    return pr;
                }
            }
        }
        bytes[i] = ci;
    }

    // try to remove a segment if we haven't removed one yet
    if !has_removed && let Some(replacements) = remove(bytes[i]) {
        let ci = bytes[i];
        for &r in replacements.as_bytes() {
            bytes[i] = r;
            if has_added {
                // We've added and removed a segment. Try to parse the equation.
                let pr = parse_equation(bytes);
                if pr.is_some() {
                    return pr;
                }
            } else {
                // continue with the next character but set the `has_removed`
                // flag to `true`
                let pr = recurse(bytes, i + 1, true, false);
                if pr.is_some() {
                    return pr;
                }
            }
        }
        bytes[i] = ci;
    }

    // try to move a segment within the current character if we haven't moved
    // anything yet
    if !has_removed
        && !has_added
        && let Some(replacements) = move_within(bytes[i])
    {
        let ci = bytes[i];
        for &r in replacements.as_bytes() {
            bytes[i] = r;
            // We've added and removed a segment. Try to parse the equation.
            let pr = parse_equation(bytes);
            if pr.is_some() {
                return pr;
            }
        }
        bytes[i] = ci;
    }

    recurse(bytes, i + 1, has_removed, has_added)
}

fn main() {
    // part 1 (note that the numbers in the input file always only have one
    // digit, so we can use the same code as for part 2)
    let input =
        fs::read_to_string("everybody_codes_e2026_q20_p1.txt").expect("Could not read file");
    println!(
        "{}",
        input
            .lines()
            .filter_map(|l| parse_equation(l.as_bytes()))
            .sum::<u64>()
    );

    // part 2
    let input =
        fs::read_to_string("everybody_codes_e2026_q20_p2.txt").expect("Could not read file");
    println!(
        "{}",
        to_radix11(
            input
                .lines()
                .filter_map(|l| parse_equation(l.as_bytes()))
                .sum::<u64>()
        )
    );

    // part 3
    let input =
        fs::read_to_string("everybody_codes_e2026_q20_p3.txt").expect("Could not read file");

    let mut total = 0;
    for l in input.lines() {
        let b = l.as_bytes();
        total += parse_equation(b)
            .or_else(|| recurse(&mut b.to_vec(), 0, false, false))
            .unwrap_or_default();
    }
    println!("{}", to_radix11(total));
}
