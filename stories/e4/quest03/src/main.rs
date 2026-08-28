use std::fs;

#[derive(Default)]
struct Configuration {
    width: i64,
    height: i64,
    horizontal_offsets: String,
    vertical_offsets: String,
}

fn parse(file: &str) -> Configuration {
    let input = fs::read_to_string(file).expect("Could not read file");

    let mut result = Configuration::default();
    for l in input.lines() {
        let (key, value) = l.split_once('=').unwrap();
        match key {
            "width" => result.width = value.parse().unwrap(),
            "height" => result.height = value.parse().unwrap(),
            "horizontal-offsets" => result.horizontal_offsets = value.to_string(),
            "vertical-offsets" => result.vertical_offsets = value.to_string(),
            _ => unreachable!("Invalid key: {key}"),
        }
    }

    result
}

fn run(config: Configuration) -> (i64, i64) {
    // in every row, the pattern repeats after `vertical_offsets.len() * 2` cols
    let cycle_right = config.vertical_offsets.len() as i64 * 2;

    // the rows repeat after `horizontal_offsets.len() * 2` rows
    let cycle_bottom = config.horizontal_offsets.len() as i64 * 2;

    let mut total_true = 0i64;
    let mut total_false = 0i64;

    // the color of the left-most cell in the previous row
    let mut first_color_of_last_row = true;

    // iterators over horizontal_offsets to determine if there are stitches at
    // the top or bottom of the current cell
    let mut hti = config.horizontal_offsets.chars().cycle();
    let mut hbi = config.horizontal_offsets.chars().cycle().skip(1);

    // iterate over all cells ...
    let mut row = 0;
    let mut row_even = true;
    while row < config.height {
        // check if all even cells in this row have a stitch at the top
        let top_even = hti.next().unwrap() == '0';

        // check if all even cells in this row have a stitch at the bottom
        let bottom_even = hbi.next().unwrap() == '0';

        // determine color of the left-most cell in this row
        let mut current_color = if top_even {
            !first_color_of_last_row
        } else {
            first_color_of_last_row
        };
        first_color_of_last_row = current_color;

        // iterators over vertical_offsets to determine if there are stitches at
        // the left or right of the current cell
        let mut vli = config.vertical_offsets.chars().cycle();
        let mut vri = config.vertical_offsets.chars().cycle().skip(1);

        // iterate over all columns in this row ...
        let mut col = 0;
        let mut col_even = true;
        let mut row_total_true = 0;
        let mut row_total_false = 0;
        while col < config.width {
            // check if all even rows in this column have a stitch at the left
            let left_even = vli.next().unwrap() == '0';

            // check if all even rows in this column have a stitch at the right
            let right_even = vri.next().unwrap() == '0';

            // determine if this cell is isolated
            let is_isolated = top_even == col_even
                && bottom_even == col_even
                && left_even == row_even
                && right_even == row_even;

            if is_isolated {
                if current_color {
                    row_total_true += 1;
                } else {
                    row_total_false += 1;
                }
            }

            // flip color if there is a stitch at the right
            if col < config.width - 1 && right_even == row_even {
                current_color = !current_color;
            }

            // increase row
            col += 1;
            col_even = !col_even;

            // skip ahead if we found a cycle
            if col < config.width && col % cycle_right == 0 {
                let reps = config.width / cycle_right;
                col = cycle_right * reps;
                row_total_true *= reps;
                row_total_false *= reps;
            }
        }

        total_true += row_total_true;
        total_false += row_total_false;

        row += 1;
        row_even = !row_even;

        // skip ahead if we found a cycle
        if row < config.height && row % cycle_bottom == 0 {
            let reps = config.height / cycle_bottom;
            row = cycle_bottom * reps;
            total_true *= reps;
            total_false *= reps;
        }
    }

    (total_true, total_false)
}

fn main() {
    // part 1
    let config = parse("everybody_codes_e4_q03_p1.txt");
    let (a, b) = run(config);
    println!("{}", a + b);

    // part 2
    let config = parse("everybody_codes_e4_q03_p2.txt");
    let (a, b) = run(config);
    println!("{}", a.max(b));

    // part 3
    let config = parse("everybody_codes_e4_q03_p3.txt");
    let (a, b) = run(config);
    println!("{}", a.max(b));
}
