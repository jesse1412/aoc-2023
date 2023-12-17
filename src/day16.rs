use std::collections::HashSet;

pub fn part1() -> u32 {
    let input = include_str!(r"inputs\day16.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let initial_beam = Beam {
        x: 0,
        y: 0,
        direction: Direction::East,
    };
    get_energize_count_from_given_beam(initial_beam, &grid)
}

pub fn part2() -> u32 {
    let input = include_str!(r"inputs\day16.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut max_energies = 0;
    // Top/bottom
    for x in 0..grid[0].len() {
        max_energies = max_energies.max(get_energize_count_from_given_beam(
            Beam {
                x,
                y: 0,
                direction: Direction::South,
            },
            &grid,
        ));
        max_energies = max_energies.max(get_energize_count_from_given_beam(
            Beam {
                x,
                y: grid.len() - 1,
                direction: Direction::North,
            },
            &grid,
        ));
    }
    for y in 0..grid.len() {
        max_energies = max_energies.max(get_energize_count_from_given_beam(
            Beam {
                x: 0,
                y,
                direction: Direction::East,
            },
            &grid,
        ));
        max_energies = max_energies.max(get_energize_count_from_given_beam(
            Beam {
                x: grid[0].len() - 1,
                y,
                direction: Direction::West,
            },
            &grid,
        ));
    }
    max_energies
}

fn get_energize_count_from_given_beam(initial_beam: Beam, grid: &[Vec<char>]) -> u32 {
    let mut beams: Vec<Beam> = vec![initial_beam];
    let mut seen_beams: HashSet<Beam> = HashSet::new();
    let mut energized: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    energized[initial_beam.y][initial_beam.x] = true;
    while let Some(beam) = beams.pop() {
        // visualize(&grid, &beam);
        if !seen_beams.contains(&beam) {
            seen_beams.insert(beam);
            match beam.direction {
                Direction::North => do_go_north_beam(grid, &beam, &mut energized, &mut beams),
                Direction::East => do_go_east_beam(grid, &beam, &mut energized, &mut beams),
                Direction::South => do_go_south_beam(grid, &beam, &mut energized, &mut beams),
                Direction::West => do_go_west_beam(grid, beam, &mut energized, &mut beams),
            }
        }
    }
    let mut result = 0;
    for row in energized {
        for b in row {
            result += b as u32;
        }
    }
    result
}

fn do_go_north_beam(
    grid: &[Vec<char>],
    beam: &Beam,
    energized: &mut [Vec<bool>],
    beams: &mut Vec<Beam>,
) {
    match grid[beam.y][beam.x] {
        '.' | '|' => {
            if let Some(new_beam) = try_make_go_north_beam(beam) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        '/' => {
            if let Some(new_beam) = try_make_go_east_beam(beam, grid) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        '\\' => {
            if let Some(new_beam) = try_make_go_west_beam(beam) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        '-' => {
            if let Some(new_beam) = try_make_go_east_beam(beam, grid) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
            if let Some(new_beam) = try_make_go_west_beam(beam) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        c => {
            panic!("Invalid character: {c}");
        }
    }
}

fn do_go_east_beam(
    grid: &[Vec<char>],
    beam: &Beam,
    energized: &mut [Vec<bool>],
    beams: &mut Vec<Beam>,
) {
    match grid[beam.y][beam.x] {
        '.' | '-' => {
            if let Some(new_beam) = try_make_go_east_beam(beam, grid) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        '/' => {
            if let Some(new_beam) = try_make_go_north_beam(beam) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        '\\' => {
            if let Some(new_beam) = try_make_go_south_beam(beam, grid) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        '|' => {
            if let Some(new_beam) = try_make_go_north_beam(beam) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
            if let Some(new_beam) = try_make_go_south_beam(beam, grid) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        c => {
            panic!("Invalid character: {c}");
        }
    }
}

fn do_go_south_beam(
    grid: &[Vec<char>],
    beam: &Beam,
    energized: &mut [Vec<bool>],
    beams: &mut Vec<Beam>,
) {
    match grid[beam.y][beam.x] {
        '.' | '|' => {
            if let Some(new_beam) = try_make_go_south_beam(beam, grid) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        '/' => {
            if let Some(new_beam) = try_make_go_west_beam(beam) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        '\\' => {
            if let Some(new_beam) = try_make_go_east_beam(beam, grid) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        '-' => {
            if let Some(new_beam) = try_make_go_east_beam(beam, grid) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
            if let Some(new_beam) = try_make_go_west_beam(beam) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        c => {
            panic!("Invalid character: {c}");
        }
    }
}

fn do_go_west_beam(
    grid: &[Vec<char>],
    beam: Beam,
    energized: &mut [Vec<bool>],
    beams: &mut Vec<Beam>,
) {
    match grid[beam.y][beam.x] {
        '.' | '-' => {
            if let Some(new_beam) = try_make_go_west_beam(&beam) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        '/' => {
            if let Some(new_beam) = try_make_go_south_beam(&beam, grid) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        '\\' => {
            if let Some(new_beam) = try_make_go_north_beam(&beam) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        '|' => {
            if let Some(new_beam) = try_make_go_north_beam(&beam) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
            if let Some(new_beam) = try_make_go_south_beam(&beam, grid) {
                energized[new_beam.y][new_beam.x] = true;
                beams.push(new_beam);
            }
        }
        c => {
            panic!("Invalid character: {c}");
        }
    }
}

fn try_make_go_north_beam(beam: &Beam) -> Option<Beam> {
    if beam.y > 0 {
        Some(Beam {
            y: beam.y - 1,
            direction: Direction::North,
            ..*beam
        })
    } else {
        None
    }
}

fn try_make_go_east_beam(beam: &Beam, grid: &[Vec<char>]) -> Option<Beam> {
    if beam.x + 1 < grid[0].len() {
        Some(Beam {
            x: beam.x + 1,
            direction: Direction::East,
            ..*beam
        })
    } else {
        None
    }
}

fn try_make_go_south_beam(beam: &Beam, grid: &[Vec<char>]) -> Option<Beam> {
    if beam.y + 1 < grid.len() {
        Some(Beam {
            y: beam.y + 1,
            direction: Direction::South,
            ..*beam
        })
    } else {
        None
    }
}

fn try_make_go_west_beam(beam: &Beam) -> Option<Beam> {
    if beam.x > 0 {
        Some(Beam {
            x: beam.x - 1,
            direction: Direction::West,
            ..*beam
        })
    } else {
        None
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn _visualize(grid: &[Vec<char>], beam: &Beam) {
    println!();
    for (y, row) in grid.iter().enumerate() {
        println!();
        for (x, c) in row.iter().enumerate() {
            if y == beam.y && x == beam.x {
                match beam.direction {
                    Direction::North => print!("^"),
                    Direction::East => print!(">"),
                    Direction::South => print!("v"),
                    Direction::West => print!("<"),
                }
            } else {
                print!("{c}");
            }
        }
    }
}
