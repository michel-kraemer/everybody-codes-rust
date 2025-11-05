use std::{
    fmt::{Display, Formatter},
    fs,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Complex {
    x: i64,
    y: i64,
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{},{}]", self.x, self.y))?;
        Ok(())
    }
}

impl From<(i64, i64)> for Complex {
    fn from(c: (i64, i64)) -> Self {
        Complex::new(c.0, c.1)
    }
}

impl<T> Add<T> for Complex
where
    T: Into<Self>,
{
    type Output = Complex;

    fn add(self, rhs: T) -> Self::Output {
        let other = rhs.into();
        Complex {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign<T> for Complex
where
    T: Into<Self>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T> Mul<T> for Complex
where
    T: Into<Self>,
{
    type Output = Complex;

    fn mul(self, rhs: T) -> Self::Output {
        let other = rhs.into();
        Complex {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }
}

impl<T> MulAssign<T> for Complex
where
    T: Into<Self>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T> Div<T> for Complex
where
    T: Into<Self>,
{
    type Output = Complex;

    fn div(self, rhs: T) -> Self::Output {
        let other = rhs.into();
        Complex {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> DivAssign<T> for Complex
where
    T: Into<Self>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl Complex {
    fn new(x: i64, y: i64) -> Complex {
        Complex { x, y }
    }
}

fn parse(file: &str) -> Complex {
    let input = fs::read_to_string(file).expect("Could not read file");
    let (x, y) = input[3..input.len() - 1].split_once(',').unwrap();
    let x = x.parse::<i64>().unwrap();
    let y = y.parse::<i64>().unwrap();
    Complex::new(x, y)
}

fn part1(mut r: Complex, a: Complex) -> Complex {
    r *= r;
    r /= (10, 10);
    r + a
}

fn is_engraved(p: Complex) -> bool {
    let mut r = Complex::new(0, 0);
    for _ in 0..100 {
        r *= r;
        r /= (100000, 100000);
        r += p;
        if !(-1000000..=1000000).contains(&r.x) || !(-1000000..=1000000).contains(&r.y) {
            return false;
        }
    }
    true
}

fn count_engraved(a: Complex, size: i64, step: i64) -> i64 {
    let mut total = 0;
    for x in 0..=size {
        for y in 0..=size {
            let p = a + Complex::new(x * step, y * step);
            if is_engraved(p) {
                total += 1;
            }
        }
    }
    total
}

fn main() {
    // part 1
    let a = parse("everybody_codes_e2025_q02_p1.txt");
    let mut r = Complex::new(0, 0);
    r = part1(r, a);
    r = part1(r, a);
    r = part1(r, a);
    println!("{r}");

    // part 2
    let a = parse("everybody_codes_e2025_q02_p2.txt");
    println!("{}", count_engraved(a, 100, 10));

    // part 3
    let a = parse("everybody_codes_e2025_q02_p3.txt");
    println!("{}", count_engraved(a, 1000, 1));
}
