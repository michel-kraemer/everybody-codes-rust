use std::fs;

use itertools::Itertools;
use skia_safe::{Color, EncodedImageFormat, Paint, surfaces};

use crate::combine;

fn write_image<T>(lines: &[T], output: &str)
where
    T: AsRef<str>,
{
    let mut surface =
        surfaces::raster_n32_premul((lines[0].as_ref().len() as i32, lines.len() as i32))
            .expect("surface");

    let mut paint = Paint::default();
    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.as_ref().chars().enumerate() {
            let color = match c {
                'C' => Color::CYAN,
                'M' => Color::MAGENTA,
                'Y' => Color::YELLOW,
                'K' => Color::BLACK,
                'R' => Color::RED,
                'G' => Color::GREEN,
                'B' => Color::BLUE,
                _ => unreachable!(),
            };
            paint.set_color(color);
            surface.canvas().draw_point((x as i32, y as i32), &paint);
        }
    }

    let image = surface.image_snapshot();
    let mut context = surface.direct_context();
    let data = image
        .encode(context.as_mut(), EncodedImageFormat::PNG, None)
        .unwrap();
    std::fs::write(output, data).unwrap();
}

pub fn visualize1() {
    let input =
        fs::read_to_string("everybody_codes_e2026_q19_p1.txt").expect("Could not read file");
    let (block1, block2) = input.trim().split_once("\n\n").unwrap();
    write_image(
        &block1.lines().collect::<Vec<_>>(),
        "everybody_codes_e2026_q19_p1a.png",
    );
    write_image(
        &block2.lines().collect::<Vec<_>>(),
        "everybody_codes_e2026_q19_p1b.png",
    );

    let mut combined = Vec::new();
    for (l1, l2) in block1.lines().zip(block2.lines()) {
        let mut lc = String::new();
        for (mut c1, mut c2) in l1.chars().zip(l2.chars()) {
            if c2 < c1 {
                (c1, c2) = (c2, c1);
            }

            let r = match (c1, c2) {
                ('K', _) | (_, 'K') => 'K',
                ('C', 'M') => 'B',
                ('C', 'Y') => 'G',
                ('M', 'Y') => 'R',
                _ => c1,
            };

            lc.push(r);
        }
        combined.push(lc);
    }

    write_image(&combined, "everybody_codes_e2026_q19_p1c.png");
}

pub fn visualize2() {
    let input =
        fs::read_to_string("everybody_codes_e2026_q19_p2.txt").expect("Could not read file");

    let (raw_block1, block2) = input.trim().split_once("\n\n").unwrap();
    let block1 = raw_block1.lines().collect::<Vec<_>>();
    write_image(&block1, "everybody_codes_e2026_q19_p2a.png");

    let block3 = raw_block1
        .bytes()
        .filter(|b| *b != b'\n')
        .collect::<Vec<_>>();
    let block2 = block2
        .bytes()
        .filter(|b| *b != b'\n')
        .map(|b| (b - b'1') as usize)
        .collect::<Vec<_>>();
    let mut max = 0;
    let mut max_permutation = Vec::new();
    for p in b"CKMY".iter().permutations(4) {
        let m = combine(&block3, block2.iter().map(|&i| *p[i]));
        if m > max {
            max = m;
            max_permutation = p;
        }
    }

    let block2 = block2
        .chunks(block2.len() / block1.len())
        .map(|i| {
            i.iter()
                .map(|j| *max_permutation[*j] as char)
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    write_image(&block2, "everybody_codes_e2026_q19_p2b.png");

    let mut combined = Vec::new();
    for (l1, l2) in block1.into_iter().zip(block2) {
        let mut lc = String::new();
        for (mut c1, mut c2) in l1.chars().zip(l2.chars()) {
            if c2 < c1 {
                (c1, c2) = (c2, c1);
            }

            let r = match (c1, c2) {
                ('K', _) | (_, 'K') => 'K',
                ('C', 'M') => 'B',
                ('C', 'Y') => 'G',
                ('M', 'Y') => 'R',
                _ => c1,
            };

            lc.push(r);
        }
        combined.push(lc);
    }

    write_image(&combined, "everybody_codes_e2026_q19_p2c.png");
}

pub fn visualize3() {
    let input =
        fs::read_to_string("everybody_codes_e2026_q19_p3.txt").expect("Could not read file");
    write_image(
        &input.trim().lines().collect::<Vec<_>>(),
        "everybody_codes_e2026_q19_p3.png",
    );
}
