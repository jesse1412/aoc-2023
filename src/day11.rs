use itertools::Itertools;
use std::collections::HashSet;

pub fn part1() -> i64 {
    let input = include_str!(r"inputs\day11.txt");
    // We represent galaxies as -1, and cost to move as unsigned ints.
    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let grid = get_physically_expanded_grid(grid);

    let mut galaxy_positions: Vec<(i64, i64)> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxy_positions.push((x as i64, y as i64));
            }
        }
    }
    let galaxy_combos = galaxy_positions
        .iter()
        .cartesian_product(galaxy_positions.iter());
    let mut total_paths: i64 = 0;
    for ((x1, y1), (x2, y2)) in galaxy_combos {
        total_paths += i64::abs(x1 - x2) + i64::abs(y1 - y2);
    }
    // We did cartesian product, so every pair was counted twice.
    total_paths / 2
}

fn get_physically_expanded_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let grid = get_physically_expanded_grid_horizontally(grid);
    // Rotate the grid.
    let grid: Vec<Vec<char>> = (0..grid[0].len())
        .map(|i| grid.iter().map(|c| c[i]).collect())
        .collect();
    // Repeate horizontal expansion.
    get_physically_expanded_grid_horizontally(grid)
}

fn get_physically_expanded_grid_horizontally(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = Vec::new();
    for line in grid.iter_mut() {
        let cost_set: HashSet<&char> = HashSet::from_iter(line.iter());
        new_grid.push(line.clone());
        if !cost_set.contains(&'#') {
            new_grid.push(line.clone());
        }
    }
    new_grid
}

pub fn part2() -> i64 {
    let input = include_str!(r"inputs\day11.txt");
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    // We represent galaxies as -1, and cost to move as unsigned ints.
    let mut grid: Vec<Vec<i32>> = input
        .lines()
        .enumerate()
        .map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '#' {
                        galaxies.push((x, y));
                        -1
                    } else {
                        1
                    }
                })
                .collect()
        })
        .collect();

    let expansion_size: i32 = 1000000;

    expand_grid_movement_costs(&mut grid, &galaxies, expansion_size);

    let minimal_galaxy_combos = get_minimal_galaxy_combos(galaxies);

    let mut total_dist_travelled: i64 = 0;
    for ((xa, ya), (xb, yb)) in minimal_galaxy_combos {
        let mut combo_dist_travelled: i64 = 0;
        for x in xa..xb {
            combo_dist_travelled += grid[ya][x] as i64;
        }
        if ya <= yb {
            for y in ya..yb {
                combo_dist_travelled += grid[y][xa] as i64;
            }
        } else {
            for y in yb..ya {
                combo_dist_travelled += grid[y][xa] as i64;
            }
        }
        total_dist_travelled += combo_dist_travelled;
    }
    total_dist_travelled
}

fn get_minimal_galaxy_combos(
    galaxies: Vec<(usize, usize)>,
) -> HashSet<((usize, usize), (usize, usize))> {
    let galaxy_combos: HashSet<(&(usize, usize), &(usize, usize))> =
        galaxies.iter().cartesian_product(galaxies.iter()).collect();
    let mut minimal_galaxy_combos: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    for (a, b) in galaxy_combos.iter() {
        if a != b {
            let (xa, ya) = a;
            let (xb, yb) = b;
            if xa < xb || (xa == xb && ya < yb) {
                minimal_galaxy_combos.insert((**a, **b));
            }
        }
    }
    minimal_galaxy_combos
}

fn expand_grid_movement_costs(
    grid: &mut [Vec<i32>],
    galaxies: &[(usize, usize)],
    expansion_size: i32,
) {
    for line in grid.iter_mut() {
        let cost_set: HashSet<&i32> = HashSet::from_iter(line.iter());
        if !cost_set.contains(&-1) {
            line.iter_mut().for_each(|v| *v = expansion_size);
        }
    }
    for col_i in 0..grid.first().unwrap().len() {
        let mut cost_set: HashSet<i32> = HashSet::new();
        for row in grid.iter_mut() {
            cost_set.insert(row[col_i]);
        }
        if !cost_set.contains(&-1) {
            for row in grid.iter_mut() {
                row[col_i] = expansion_size;
            }
        }
    }
    for (x, y) in galaxies.iter() {
        // Cost of moving through galaxies is always 1.
        grid[*y][*x] = 1;
    }
}
