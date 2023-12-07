use std::fmt::Debug;

pub fn part1() -> u64 {
    let input = include_str!(r"inputs\day6.txt");
    let games = get_games(input);
    // Equation:
    // (time - winding time) * winding_time > distance
    // re-arrange: -winding_time^2 + time * winding_time > distance.
    // Known: time, distance.
    // Quadratic equation to solve.
    solve(games)
}

pub fn part2() -> u64 {
    let input = include_str!(r"inputs\day6.txt");
    let games = get_games2(input);
    solve(games)
}

fn solve(games: Vec<Game>) -> u64 {
    let mut solutions: Vec<(i64, i64)> = Vec::new();
    for game in games {
        let a = -1_f64;
        let b = game.time as f64;
        let c = -game.distance as f64;
        let mut solution_1: i64 =
            ((-b + (b.powi(2) - 4_f64 * a * c).powf(0.5)) / (2_f64 * a)).ceil() as i64;
        let mut solution_2: i64 =
            ((-b - (b.powi(2) - 4_f64 * a * c).powf(0.5)) / (2_f64 * a)).floor() as i64;
        if solution_1 * (game.time - solution_1) == game.distance {
            solution_1 += 1
        }
        if solution_2 * (game.time - solution_2) == game.distance {
            solution_2 -= 1
        }
        solutions.push((solution_1, solution_2));
    }
    solutions
        .iter()
        .fold(1, |acc, x| acc * (x.1 as u64 + 1 - x.0 as u64))
}

fn get_games(input: &str) -> Vec<Game> {
    let times: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let distances: Vec<i64> = input
        .lines()
        .nth(1)
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut games: Vec<Game> = Vec::new();
    for (time, distance) in times.iter().zip(distances.iter()) {
        games.push(Game {
            time: *time,
            distance: *distance,
        })
    }
    games
}

fn get_games2(input: &str) -> Vec<Game> {
    let times: Vec<i64> = vec![input
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<i64>()
        .unwrap()];
    let distances: Vec<i64> = vec![input
        .lines()
        .nth(1)
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<i64>()
        .unwrap()];
    let mut games: Vec<Game> = Vec::new();
    for (time, distance) in times.iter().zip(distances.iter()) {
        games.push(Game {
            time: *time,
            distance: *distance,
        })
    }
    games
}

#[derive(Debug)]
struct Game {
    time: i64,
    distance: i64,
}
