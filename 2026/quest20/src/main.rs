use std::{fs, range::Range};

struct Equation {
    parts: [i64; 7],
    raw: Vec<u8>,
    ranges: [Range<usize>; 7],
}

impl Equation {
    fn from(s: &str) -> Self {
        let end_a = s.find(' ').unwrap();
        let end_b = end_a + 3 + s[end_a + 3..].find(' ').unwrap();
        let end_c = end_b + 3 + s[end_b + 3..].find(' ').unwrap();
        let range_a = Range::from(0..end_a);
        let range_op1 = Range::from(end_a + 1..end_a + 2);
        let range_b = Range::from(end_a + 3..end_b);
        let range_eq = Range::from(end_b + 1..end_b + 2);
        let range_c = Range::from(end_b + 3..end_c);
        let range_op2 = Range::from(end_c + 1..end_c + 2);
        let range_d = Range::from(end_c + 3..s.len());
        let raw = s.bytes().collect::<Vec<_>>();
        let op1 = (raw[end_a + 1] == b'+') as i64;
        let op2 = (raw[end_c + 1] == b'+') as i64;
        let a = i64::from_str_radix(&s[range_a], 11).unwrap();
        let b = i64::from_str_radix(&s[range_b], 11).unwrap();
        let c = i64::from_str_radix(&s[range_c], 11).unwrap();
        let d = i64::from_str_radix(&s[range_d], 11).unwrap();
        Self {
            parts: [a, op1, b, 0, c, op2, d],
            raw,
            ranges: [
                range_a, range_op1, range_b, range_eq, range_c, range_op2, range_d,
            ],
        }
    }

    /// Check if this equation is correct
    fn is_correct(&self) -> bool {
        let left = if self.parts[1] == 1 {
            self.parts[0] + self.parts[2]
        } else {
            self.parts[0] - self.parts[2]
        };
        let right = if self.parts[5] == 1 {
            self.parts[4] + self.parts[6]
        } else {
            self.parts[4] - self.parts[6]
        };
        left == right
    }

    /// Count the number of segments in this equation
    fn count_segments(&self) -> u64 {
        // Equation is correct. Count segments.
        let mut total = count_segments(&self.raw[self.ranges[0]]);
        total += count_segments(&self.raw[self.ranges[2]]);
        total += count_segments(&self.raw[self.ranges[4]]);
        total += count_segments(&self.raw[self.ranges[6]]);

        // operators
        total += self.parts[1] as u64 + 1;
        total += self.parts[5] as u64 + 1;

        // '=' sign
        total += 2;

        total
    }

    /// Replace a byte at position i in the raw equation and in part j
    fn replace(&mut self, i: usize, j: usize, r: u8) {
        self.raw[i] = r;
        if j == 1 || j == 5 {
            self.parts[j] = (r == b'+') as i64;
        } else {
            // update a digit in the base 11 representation of self.parts[j]
            let s = (if r == b'A' { 10 } else { r - b'0' }) as i64;
            let k = (self.ranges[j].end - i - 1) as u32;
            let powk = 11_i64.pow(k);
            let powk1 = powk * 11;
            self.parts[j] =
                self.parts[j] - (self.parts[j] % powk1 - self.parts[j] % powk) + s * powk;
        }
    }
}

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

/// Add a segment to a character and return a list of possible replacement
/// characters. Return `None` if there are not valid replacement characters.
fn add(b: u8) -> Option<&'static str> {
    match b {
        b'2' | b'4' | b'7' | b'8' | b'+' | b'=' => None,
        b'0' | b'6' | b'9' | b'A' => Some("8"),
        b'1' => Some("7"),
        b'3' => Some("9"),
        b'5' => Some("69"),
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
        b'2' | b'5' => Some("3"),
        b'3' => Some("25"),
        b'6' => Some("09A"),
        b'9' => Some("06A"),
        b'A' => Some("069"),
        _ => unreachable!(),
    }
}

/// Recursively try to move a single segment within the given broken equation
/// and return the number of segments if the equation can be fixed
fn recurse(
    equation: &mut Equation,
    i: usize,
    j: usize,
    has_removed: bool,
    has_added: bool,
) -> Option<u64> {
    if i == equation.raw.len() {
        // unable to fix equation
        return None;
    }

    // skip whitespace characters and equals sign
    if equation.raw[i].is_ascii_whitespace() {
        let (new_i, new_j) = if j == 2 {
            (i + 3, j + 2)
        } else {
            (i + 1, j + 1)
        };
        return recurse(equation, new_i, new_j, has_removed, has_added);
    }

    // try to add a segment if we haven't added one yet
    if !has_added && let Some(replacements) = add(equation.raw[i]) {
        let ci = equation.raw[i];
        let pj = equation.parts[j];
        for &r in replacements.as_bytes() {
            equation.replace(i, j, r);
            if has_removed {
                // We've added and removed a segment. Try to parse the equation.
                if equation.is_correct() {
                    return Some(equation.count_segments());
                }
            } else {
                // continue with the next character but set the `has_added` flag
                // to `true`
                let pr = recurse(equation, i + 1, j, false, true);
                if pr.is_some() {
                    return pr;
                }
            }
        }
        equation.raw[i] = ci;
        equation.parts[j] = pj;
    }

    // try to remove a segment if we haven't removed one yet
    if !has_removed && let Some(replacements) = remove(equation.raw[i]) {
        let ci = equation.raw[i];
        let pj = equation.parts[j];
        for &r in replacements.as_bytes() {
            equation.replace(i, j, r);
            if has_added {
                // We've added and removed a segment. Try to parse the equation.
                if equation.is_correct() {
                    return Some(equation.count_segments());
                }
            } else {
                // continue with the next character but set the `has_removed`
                // flag to `true`
                let pr = recurse(equation, i + 1, j, true, false);
                if pr.is_some() {
                    return pr;
                }
            }
        }
        equation.raw[i] = ci;
        equation.parts[j] = pj;
    }

    // try to move a segment within the current character if we haven't moved
    // anything yet
    if !has_removed
        && !has_added
        && let Some(replacements) = move_within(equation.raw[i])
    {
        let ci = equation.raw[i];
        let pj = equation.parts[j];
        for &r in replacements.as_bytes() {
            equation.replace(i, j, r);
            // We've added and removed a segment. Try to parse the equation.
            if equation.is_correct() {
                return Some(equation.count_segments());
            }
        }
        equation.raw[i] = ci;
        equation.parts[j] = pj;
    }

    recurse(equation, i + 1, j, has_removed, has_added)
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
            .map(Equation::from)
            .filter(|e| e.is_correct())
            .map(|e| e.count_segments())
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
                .map(Equation::from)
                .filter(|e| e.is_correct())
                .map(|e| e.count_segments())
                .sum::<u64>()
        )
    );

    // part 3
    let input =
        fs::read_to_string("everybody_codes_e2026_q20_p3.txt").expect("Could not read file");

    let mut total = 0;
    for l in input.lines() {
        let mut e = Equation::from(l);
        if e.is_correct() {
            total += e.count_segments();
        } else if let Some(c) = recurse(&mut e, 0, 0, false, false) {
            total += c;
        }
    }
    println!("{}", to_radix11(total));
}
