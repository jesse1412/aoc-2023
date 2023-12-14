pub fn part1() -> usize {
    // Go through row by row.
    // If two rows are equal, expand as far as possible.
    // Rotate board and repeat.
    let input_raw = include_str!(r"inputs\day13.txt");
    let inputs = parse_input(input_raw);
    let mut result = 0;
    for grid in inputs {
        let mirror_row_top = get_mirror_horizontal_position(&grid);
        if let Some(mirror_row_top) = mirror_row_top {
            result += mirror_row_top * 100;
        } else {
            let grid = rotate_2d_grid(grid);
            let mirror_col_top = get_mirror_horizontal_position(&grid);
            if let Some(mirror_col_top) = mirror_col_top {
                result += mirror_col_top;
            }
        }
    }
    result
}

pub fn part2() -> usize {
    // Go through row by row.
    // If two rows are equal, expand as far as possible.
    // Rotate board and repeat.
    let input_raw = include_str!(r"inputs\day13.txt");
    let inputs = parse_input(input_raw);
    let mut result = 0;
    for grid in inputs {
        let mirror_row_top = get_mirror_horizontal_position2(&grid);
        if let Some(mirror_row_top) = mirror_row_top {
            result += mirror_row_top * 100;
        } else {
            let grid = rotate_2d_grid(grid);
            let mirror_col_top = get_mirror_horizontal_position2(&grid);
            if let Some(mirror_col_top) = mirror_col_top {
                result += mirror_col_top;
            }
        }
    }
    result
}

fn parse_input(input_raw: &str) -> Vec<Vec<Vec<char>>> {
    let split_inputs = input_raw.split("\r\n\r\n");
    let mut parsed_inputs: Vec<Vec<Vec<char>>> = Vec::new();
    for input in split_inputs {
        parsed_inputs.push(
            input
                .lines()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect(),
        );
    }
    parsed_inputs
}

fn get_mirror_horizontal_position(grid: &[Vec<char>]) -> Option<usize> {
    let mut mirror_row_top: usize;
    let mut mirror_row_bottom: usize;
    for (i, (row, next_row)) in grid.iter().zip(grid.iter().skip(1)).enumerate() {
        if row == next_row {
            mirror_row_top = i;
            mirror_row_bottom = i + 1;
            let rows_top_mirror = grid[0..mirror_row_top + 1].iter().rev();
            let rows_bottom_mirror = grid[mirror_row_bottom..].iter();
            let mirror_zip = rows_top_mirror.zip(rows_bottom_mirror);
            let mut mismatch = false;
            for (top, bottom) in mirror_zip {
                if top != bottom {
                    mismatch = true;
                    break;
                }
            }
            if !mismatch {
                return Some(mirror_row_bottom);
            }
        }
    }
    None
}

fn get_mirror_horizontal_position2(grid: &[Vec<char>]) -> Option<usize> {
    let mut mirror_row_top: usize;
    let mut mirror_row_bottom: usize;
    for (i, (row, next_row)) in grid.iter().zip(grid.iter().skip(1)).enumerate() {
        if get_mismatch_amount(row, next_row) <= 1 {
            mirror_row_top = i;
            mirror_row_bottom = i + 1;
            let rows_top_mirror = grid[0..mirror_row_top + 1].iter().rev();
            let rows_bottom_mirror = grid[mirror_row_bottom..].iter();
            let mirror_zip = rows_top_mirror.zip(rows_bottom_mirror);
            let mut mismatch = false;
            let mut fixed_off_by_1 = false;
            for (top, bottom) in mirror_zip {
                let off_amount = get_mismatch_amount(top, bottom);
                if (off_amount == 1 && fixed_off_by_1) || off_amount > 1 {
                    mismatch = true;
                    break;
                } else if off_amount == 1 {
                    fixed_off_by_1 = true;
                }
            }
            if !mismatch && fixed_off_by_1 {
                return Some(mirror_row_bottom);
            }
        }
    }
    None
}

fn get_mismatch_amount(a: &[char], b: &[char]) -> usize {
    let mut mismatches = 0;
    for (a, b) in a.iter().zip(b.iter()) {
        if a != b {
            mismatches += 1;
        }
    }
    mismatches
}

fn rotate_2d_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..grid[0].len())
        .map(|i| grid.iter().rev().map(|c| c[i]).collect())
        .collect()
}
