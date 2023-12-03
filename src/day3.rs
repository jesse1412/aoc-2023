use std::collections::BTreeSet;
use std::collections::HashSet;

pub fn part1() -> u32 {
    let input = include_str!("inputs/day3.txt");
    let mut grid = get_grid_from(input);
    let symbol_adjacent_positions = get_symbol_adjacent_positions(&grid);
    let included_nums = get_all_included_nums(symbol_adjacent_positions, &mut grid);
    let mut res = 0;
    // println!("{:?}", included_nums);
    for num in included_nums {
        res += num.number;
    }
    res
}

pub fn part2() -> u32 {
    let input = include_str!("inputs/day3.txt");
    let mut grid = get_grid_from(input);
    let symbol_adjacent_positions = get_gear_adjacent_positions(&grid);
    let included_nums = get_all_included_nums(symbol_adjacent_positions, &mut grid);

    let gear_positions = get_gear_positions(&grid);

    let mut new_grid: Vec<Vec<Option<GridNumber>>> = Vec::new();
    for row in grid.iter() {
        new_grid.push(Vec::new());
        for _ in row.iter() {
            new_grid.last_mut().unwrap().push(None)
        }
    }

    for included_num in included_nums.iter() {
        for num in 0..included_num.number_length {
            new_grid[included_num.y][included_num.start_x + num] = Some(*included_num);
        }
    }

    let mut ratio_sum: u32 = 0;
    let max_x = grid.last().unwrap_or(&Vec::new()).len();
    let max_y = grid.len();
    for (x, y) in gear_positions {
        let mut surrounding_nums: HashSet<u32> = HashSet::new();
        if x > 0 {
            if let Some(num) = new_grid[y][x - 1] {
                surrounding_nums.insert(num.number);
            }
        }
        if x < max_x - 1 {
            if let Some(num) = new_grid[y][x + 1] {
                surrounding_nums.insert(num.number);
            }
        }
        if y > 0 {
            if let Some(num) = new_grid[y - 1][x] {
                surrounding_nums.insert(num.number);
            }
        }
        if y < max_y - 1 {
            if let Some(num) = new_grid[y + 1][x] {
                surrounding_nums.insert(num.number);
            }
        }
        if x > 0 && y > 0 {
            if let Some(num) = new_grid[y - 1][x - 1] {
                surrounding_nums.insert(num.number);
            }
        }
        if x < max_x - 1 && y > 0 {
            if let Some(num) = new_grid[y - 1][x + 1] {
                surrounding_nums.insert(num.number);
            }
        }
        if x > 0 && y < max_y - 1 {
            if let Some(num) = new_grid[y + 1][x - 1] {
                surrounding_nums.insert(num.number);
            }
        }
        if x < max_x - 1 && y < max_y - 1 {
            if let Some(num) = new_grid[y + 1][x + 1] {
                surrounding_nums.insert(num.number);
            }
        }

        if surrounding_nums.len() == 2 {
            ratio_sum += surrounding_nums.iter().product::<u32>();
        }
    }
    ratio_sum
}

fn get_gear_positions(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut gear_positions: Vec<(usize, usize)> = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '*' {
                gear_positions.push((x, y));
            }
        }
    }
    gear_positions
}

fn get_symbol_adjacent_positions(grid: &Vec<Vec<char>>) -> BTreeSet<(usize, usize)> {
    let mut symbol_adjacent_positions: BTreeSet<(usize, usize)> = BTreeSet::new();
    let max_x = grid.last().unwrap_or(&Vec::new()).len();
    let max_y = grid.len();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            insert_symbol_adjacent_positions(c, x, &mut symbol_adjacent_positions, y, max_x, max_y);
        }
    }
    symbol_adjacent_positions
}

fn get_gear_adjacent_positions(grid: &Vec<Vec<char>>) -> BTreeSet<(usize, usize)> {
    let mut symbol_adjacent_positions: BTreeSet<(usize, usize)> = BTreeSet::new();
    let max_x = grid.last().unwrap_or(&Vec::new()).len();
    let max_y = grid.len();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            insert_gear_adjacent_positions(c, x, &mut symbol_adjacent_positions, y, max_x, max_y);
        }
    }
    symbol_adjacent_positions
}

fn insert_symbol_adjacent_positions(
    c: &char,
    x: usize,
    symbol_adjacent_positions: &mut BTreeSet<(usize, usize)>,
    y: usize,
    max_x: usize,
    max_y: usize,
) {
    if !(c.is_numeric() || *c == '.' || *c == '\n') {
        insert_adjacent_positions(x, symbol_adjacent_positions, y, max_x, max_y);
    }
}

fn insert_gear_adjacent_positions(
    c: &char,
    x: usize,
    symbol_adjacent_positions: &mut BTreeSet<(usize, usize)>,
    y: usize,
    max_x: usize,
    max_y: usize,
) {
    if *c == '*' {
        insert_adjacent_positions(x, symbol_adjacent_positions, y, max_x, max_y);
    }
}

fn insert_adjacent_positions(
    x: usize,
    adjacent_positions: &mut BTreeSet<(usize, usize)>,
    y: usize,
    max_x: usize,
    max_y: usize,
) {
    if x > 0 {
        adjacent_positions.insert((x - 1, y));
    }
    if x < max_x - 1 {
        adjacent_positions.insert((x + 1, y));
    }
    if y > 0 {
        adjacent_positions.insert((x, y - 1));
    }
    if y < max_y - 1 {
        adjacent_positions.insert((x, y + 1));
    }
    if x > 0 && y > 0 {
        adjacent_positions.insert((x - 1, y - 1));
    }
    if x < max_x - 1 && y > 0 {
        adjacent_positions.insert((x + 1, y - 1));
    }
    if x > 0 && y < max_y - 1 {
        adjacent_positions.insert((x - 1, y + 1));
    }
    if x < max_x - 1 && y < max_y - 1 {
        adjacent_positions.insert((x + 1, y + 1));
    }
}

fn get_grid_from(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(Vec::new());
        for c in line.chars() {
            grid.last_mut().unwrap().push(c);
        }
    }
    grid
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
struct GridNumber {
    start_x: usize,
    y: usize,
    number: u32,
    number_length: usize,
}

fn get_all_included_nums(
    mut symbol_adjacent_positions: BTreeSet<(usize, usize)>,
    grid: &mut Vec<Vec<char>>,
) -> BTreeSet<GridNumber> {
    // Why BTreeSet instead of Hashset you ask?
    // BTreeSet here is sorted by x then y.
    // That means we always start as far left as possible.
    // This minimises the amount we have to go left to get the start pos of a number.
    // Also makes it MUCH easier to debug as numbers are in the order that they appear.
    let mut included_nums: BTreeSet<GridNumber> = BTreeSet::new();
    while !symbol_adjacent_positions.is_empty() {
        if let Some(num) = get_next_included_num(grid, &mut symbol_adjacent_positions) {
            included_nums.insert(num);
        }
    }
    included_nums
}

fn get_next_included_num(
    grid: &Vec<Vec<char>>,
    symbol_adjacent_positions: &mut BTreeSet<(usize, usize)>,
) -> Option<GridNumber> {
    if symbol_adjacent_positions.is_empty() {
        return None;
    }
    let (mut x, y) = symbol_adjacent_positions.pop_first().unwrap();
    if !grid[y][x].is_numeric() {
        return None;
    }
    // Find the start of the number.
    while x > 0 && grid[y][x - 1].is_numeric() {
        x -= 1;
    }
    let start_x = x;
    let mut num_str = String::new();
    // Build the number up.
    while x < grid.len() && grid[y][x].is_numeric() {
        num_str.push(grid[y][x]);
        // Remove the already tried ones.
        symbol_adjacent_positions.remove(&(x, y));
        x += 1;
    }
    Some(GridNumber {
        start_x,
        y,
        number: num_str.parse().unwrap(),
        number_length: x - start_x,
    })
}
