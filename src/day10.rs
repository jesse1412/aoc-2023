use std::iter::repeat;

use counter::Counter;

pub fn part1() -> usize {
    let input = include_str!(r"inputs\day10.txt");
    let mut grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut start_x = 0;
    let mut start_y = 0;
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, c) in row.iter_mut().enumerate() {
            if *c == 'S' {
                start_x = x;
                start_y = y;
                *c = 0 as char;
                break;
            }
        }
    }
    let mut curr_positions = get_second_positions(&grid, start_y, start_x).0;
    let mut prev_positions: Vec<Position> = Vec::new();
    for _ in 0..curr_positions.len() {
        prev_positions.push(Position {
            x: start_x,
            y: start_y,
            distance: 0,
        });
    }

    loop {
        for (pos, prev_pos) in curr_positions.iter_mut().zip(prev_positions.iter_mut()) {
            let tmp = pos.clone();
            let c = grid[pos.y][pos.x];
            do_move(c, pos, prev_pos);
            pos.distance += 1;
            *prev_pos = tmp;
        }
        let pos_counter: Counter<&Position> = Counter::from_iter(curr_positions.iter());
        let ordered_counts = pos_counter.most_common();
        let (most_common_pos, most_common_count) = ordered_counts.first().unwrap();
        if *most_common_count == 2_usize {
            return most_common_pos.distance;
        }
    }
}

pub fn part2() -> usize {
    let input = include_str!(r"inputs\day10.txt");
    let mut grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut start_x = 0;
    let mut start_y = 0;
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, c) in row.iter_mut().enumerate() {
            if *c == 'S' {
                start_x = x;
                start_y = y;
                *c = 0 as char;
                break;
            }
        }
    }
    let (mut curr_positions, true_start_char) = get_second_positions(&grid, start_y, start_x);
    let mut prev_positions: Vec<Position> = Vec::new();
    for _ in 0..curr_positions.len() {
        prev_positions.push(Position {
            x: start_x,
            y: start_y,
            distance: 0,
        });
    }

    let mut new_grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..grid.len() {
        new_grid.push(repeat('.').take(grid.first().unwrap().len()).collect());
    }
    'outer: loop {
        for (pos, prev_pos) in curr_positions.iter_mut().zip(prev_positions.iter_mut()) {
            let tmp = pos.clone();
            let c = grid[pos.y][pos.x];
            new_grid[pos.y][pos.x] = c;
            do_move(c, pos, prev_pos);
            pos.distance += 1;
            *prev_pos = tmp;
        }
        let pos_counter: Counter<&Position> = Counter::from_iter(curr_positions.iter());
        let ordered_counts = pos_counter.most_common();
        let (pos, most_common_count) = ordered_counts.first().unwrap();
        if *most_common_count == 2_usize {
            new_grid[pos.y][pos.x] = grid[pos.y][pos.x];
            break 'outer;
        }
    }
    new_grid[start_y][start_x] = true_start_char;
    let mut inside_count = 0;
    for row in new_grid.iter() {
        let mut last_corner_seen = '.';
        let mut inside = false;
        for c in row.iter() {
            if *c == '|'
                || (last_corner_seen == 'L' && *c == '7')
                || (last_corner_seen == 'F' && *c == 'J')
            {
                inside = !inside;
                last_corner_seen = '.';
            } else {
                match *c {
                    '7' | 'L' | 'J' | 'F' => last_corner_seen = *c,
                    _ => (),
                }
            }
            if inside && *c == '.' {
                inside_count += 1;
            }
        }
    }
    inside_count
}

fn do_move(c: char, pos: &mut Position, prev_pos: &mut Position) {
    match c {
        '|' => {
            if pos.y > prev_pos.y {
                pos.y += 1;
            } else {
                pos.y -= 1;
            }
        }
        '-' => {
            if pos.x > prev_pos.x {
                pos.x += 1;
            } else {
                pos.x -= 1;
            }
        }
        'L' => {
            if pos.y == prev_pos.y {
                pos.y -= 1;
            } else {
                pos.x += 1
            }
        }
        'J' => {
            if pos.y == prev_pos.y {
                pos.y -= 1;
            } else {
                pos.x -= 1;
            }
        }
        '7' => {
            if pos.y == prev_pos.y {
                pos.y += 1;
            } else {
                pos.x -= 1;
            }
        }
        'F' => {
            if pos.y == prev_pos.y {
                pos.y += 1;
            } else {
                pos.x += 1;
            }
        }
        _ => panic!("Nononono"),
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
    distance: usize,
}

fn get_second_positions(
    grid: &[Vec<char>],
    start_y: usize,
    start_x: usize,
) -> (Vec<Position>, char) {
    let mut possible_second_positions: Vec<Position> = Vec::new();
    let (mut n, mut e, mut s, mut w) = (false, false, false, false);
    if grid
        .get(start_y + 1)
        .and_then(|row| row.get(start_x))
        .is_some_and(|c| ['|', 'L', 'J'].contains(c))
    {
        s = true;
        possible_second_positions.push(Position {
            x: start_x,
            y: start_y + 1,
            distance: 1_usize,
        })
    }
    if grid
        .get(start_y.checked_add_signed(-1).unwrap_or(grid.len()))
        .and_then(|row| row.get(start_x))
        .is_some_and(|c| ['|', '7', 'F'].contains(c))
    {
        n = true;
        possible_second_positions.push(Position {
            x: start_x,
            y: start_y - 1,
            distance: 1_usize,
        })
    }
    if grid
        .get(start_y)
        .and_then(|row| row.get(start_x + 1))
        .is_some_and(|c| ['-', '7', 'J'].contains(c))
    {
        e = true;
        possible_second_positions.push(Position {
            x: start_x + 1,
            y: start_y,
            distance: 1_usize,
        })
    }
    if grid
        .get(start_y)
        .and_then(|row| row.get(start_x.checked_add_signed(-1).unwrap_or(row.len())))
        .is_some_and(|c| ['-', 'F', 'L'].contains(c))
    {
        w = true;
        possible_second_positions.push(Position {
            x: start_x - 1,
            y: start_y,
            distance: 1_usize,
        })
    }
    let starting_char = match (n, e, s, w) {
        (true, true, false, false) => 'L',
        (true, false, false, true) => 'J',
        (false, true, true, false) => 'F',
        (false, false, true, true) => '7',
        (true, false, true, false) => '|',
        (false, true, false, true) => '-',
        _ => panic!("Can't see a direction"),
    };
    (possible_second_positions, starting_char)
}
