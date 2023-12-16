use blake3::hash;
use std::collections::HashMap;

pub fn part1() -> u32 {
    let input = include_str!(r"inputs\day14.txt");
    let mut grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    roll_north(&mut grid);

    calculate_load(grid)
}

pub fn part2(cycle_count: u64) -> u32 {
    let input = include_str!(r"inputs\day14.txt");
    let mut next_cycle_grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    // Only really need to loop until a cycle leads to a repeat.
    // We also need to know the time between repeats and the time until the repeating cycle started.
    // With this, we can calculate the output at any number of cycles.
    // First problem, grids are big, we don't want to store them.
    // By default rust stores the key to hashmaps for collision resolution.
    // We will instead store the hashes themselves and ignore collisions.
    // We'll use blake3 to handle the hashing (fast and insignificant collision rate).
    let mut seen: HashMap<blake3::Hash, usize> = HashMap::new();
    let mut cycles = 0;
    while !seen.contains_key(&hash(&get_grid_bytes(&next_cycle_grid))) {
        seen.insert(hash(&get_grid_bytes(&next_cycle_grid)), cycles);
        for _ in 0..4 {
            roll_north(&mut next_cycle_grid);
            next_cycle_grid = rotate_2d_grid(next_cycle_grid);
        }
        cycles += 1;
    }

    let cycles_needed_after_repeats_until_at_target =
        calculate_leftover_cycles_to_target_after_repeats(
            seen,
            &next_cycle_grid,
            cycles,
            cycle_count,
        );
    // We know we're at the start of a cycle.
    // So now we just do the needed repeats.
    for _ in 0..cycles_needed_after_repeats_until_at_target {
        for _ in 0..4 {
            roll_north(&mut next_cycle_grid);
            next_cycle_grid = rotate_2d_grid(next_cycle_grid);
        }
    }

    calculate_load(next_cycle_grid)
}

fn calculate_leftover_cycles_to_target_after_repeats(
    seen: HashMap<blake3::Hash, usize>,
    next_cycle_grid: &[Vec<char>],
    cycles: usize,
    cycle_count: u64,
) -> u64 {
    let cycle_where_repeating_starts = seen.get(&hash(&get_grid_bytes(next_cycle_grid))).unwrap();
    let cycles_between_repeats = (cycles - cycle_where_repeating_starts) as u64;
    let cycles_until_at_target = cycle_count - cycles as u64;
    cycles_until_at_target % cycles_between_repeats
}

fn get_grid_bytes(grid: &[Vec<char>]) -> Vec<u8> {
    grid.iter()
        .flat_map::<Vec<u8>, _>(|row| row.iter().collect::<String>().as_bytes().to_vec())
        .collect()
}

fn roll_north(grid: &mut Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != 'O' {
                continue;
            }
            let mut curr_grid_y = y;
            while grid
                .get(curr_grid_y.checked_add_signed(-1).unwrap_or(grid.len()))
                .is_some_and(|row| row[x] == '.')
            {
                grid[curr_grid_y][x] = '.';
                grid[curr_grid_y - 1][x] = 'O';
                curr_grid_y -= 1;
            }
        }
    }
}

fn calculate_load(grid: Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    for (y, row) in grid.iter().enumerate() {
        for c in row.iter() {
            if *c == 'O' {
                result += (grid.len() - y) as u32;
            }
        }
    }
    result
}

fn rotate_2d_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..grid[0].len())
        .map(|i| grid.iter().rev().map(|c| c[i]).collect())
        .collect()
}
