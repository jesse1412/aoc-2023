use itertools::Itertools;
use std::collections::HashSet;

pub fn day11(expansion_size: i64) -> i64 {
    let expansion_size = expansion_size - 1;
    let input = include_str!(r"inputs\day11.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let should_expand_rows = get_whether_rows_should_expand(&grid);
    let expand_rows_rolling_sum = rolling_sum(should_expand_rows);

    let should_expand_cols = get_whether_cols_should_expand(&grid);
    let expand_cols_rolling_sum: Vec<i64> = rolling_sum(should_expand_cols);

    let galaxy_positions = get_galaxy_positions(
        grid,
        expand_cols_rolling_sum,
        expansion_size,
        expand_rows_rolling_sum,
    );

    let galaxy_combos = galaxy_positions
        .iter()
        .cartesian_product(galaxy_positions.iter());
    let mut total_dist: i64 = 0;
    for ((x1, y1), (x2, y2)) in galaxy_combos {
        total_dist += i64::abs(x1 - x2) + i64::abs(y1 - y2);
    }
    // We did cartesian product, so every pair was counted twice.
    total_dist / 2
}

fn get_galaxy_positions(
    grid: Vec<Vec<char>>,
    expand_cols_rolling_sum: Vec<i64>,
    expansion_size: i64,
    expand_rows_rolling_sum: Vec<i64>,
) -> Vec<(i64, i64)> {
    let mut galaxy_positions: Vec<(i64, i64)> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxy_positions.push((
                    x as i64 + expand_cols_rolling_sum[x] * expansion_size,
                    y as i64 + expand_rows_rolling_sum[y] * expansion_size,
                ));
            }
        }
    }
    galaxy_positions
}

fn get_whether_cols_should_expand(grid: &[Vec<char>]) -> Vec<bool> {
    let mut should_expand_cols: Vec<bool> = Vec::new();
    for col_i in 0..grid.first().unwrap().len() {
        let mut cost_set: HashSet<char> = HashSet::new();
        for row in grid.iter() {
            cost_set.insert(row[col_i]);
        }
        if !cost_set.contains(&'#') {
            should_expand_cols.push(true);
        } else {
            should_expand_cols.push(false);
        }
    }
    should_expand_cols
}

fn rolling_sum(should_expand_rows: Vec<bool>) -> Vec<i64> {
    let expand_rows_rolling_sum: Vec<i64> = should_expand_rows
        .iter()
        .scan(0, |acc, i| {
            *acc += *i as i64;
            Some(*acc)
        })
        .collect();
    expand_rows_rolling_sum
}

fn get_whether_rows_should_expand(grid: &[Vec<char>]) -> Vec<bool> {
    let mut should_expand_rows: Vec<bool> = Vec::new();
    for row in grid.iter() {
        let cost_set: HashSet<&char> = HashSet::from_iter(row);
        if !cost_set.contains(&'#') {
            should_expand_rows.push(true)
        } else {
            should_expand_rows.push(false)
        }
    }
    should_expand_rows
}
