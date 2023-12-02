const REQUIRED_RED: u32 = 12;
const REQUIRED_GREEN: u32 = 13;
const REQUIRED_BLUE: u32 = 14;

pub fn part1() -> u32 {
    let input = include_str!("inputs/day2.txt");
    let mut sum_of_passing_ids: u32 = 0;
    for (i, line) in input.lines().enumerate() {
        let (max_red, max_green, max_blue) = get_max_rgb(line);
        if max_red <= REQUIRED_RED && max_green <= REQUIRED_GREEN && max_blue <= REQUIRED_BLUE {
            sum_of_passing_ids += i as u32 + 1;
        }
    }
    sum_of_passing_ids
}

pub fn part2() -> u32 {
    let input = include_str!("inputs/day2.txt");
    let mut sum_of_min_powers: u32 = 0;
    for line in input.lines() {
        let (max_red, max_green, max_blue) = get_max_rgb(line);
        sum_of_min_powers += max_red * max_green * max_blue;
    }
    sum_of_min_powers
}

fn get_max_rgb(line: &str) -> (u32, u32, u32) {
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;
    let games_start = line.find(':').unwrap();
    let games_slice = &line[games_start + 2..].trim();
    let games = games_slice.split(';');
    for game in games {
        let game = game.trim();
        let amt_cols = game.split(',');
        for amt_col in amt_cols {
            let amt_col = amt_col.trim();
            let (amt, colour) = amt_col.split_once(' ').unwrap();
            let colour = colour.trim();
            match colour {
                "red" => max_red = max_red.max(amt.parse().unwrap()),
                "green" => max_green = max_green.max(amt.parse().unwrap()),
                "blue" => max_blue = max_blue.max(amt.parse().unwrap()),
                _ => (),
            }
        }
    }
    (max_red, max_green, max_blue)
}
