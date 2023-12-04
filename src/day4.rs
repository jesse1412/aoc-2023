use std::collections::BTreeSet;

pub fn part1() -> u32 {
    let input = include_str!(r"inputs/day4.txt");
    let mut total_won = 0;
    for line in input.lines() {
        total_won += get_line_score(line).winning_value;
    }
    total_won
}

pub fn part2() -> u32 {
    let input = include_str!(r"inputs/day4.txt");
    let mut line_results: Vec<LineResult> = Vec::new();
    for line in input.lines() {
        line_results.push(get_line_score(line));
    }
    let mut card_counts = vec![0; line_results.len()];
    for (i, line) in line_results.iter().enumerate() {
        let start: usize = i + 1;
        let end: usize = (i + 1 + line.win_count).min(line_results.len() + line.win_count);
        card_counts[i] += 1;
        for j in start..end {
            card_counts[j] += card_counts[i];
        }
    }
    card_counts.iter().sum()
}

struct LineResult {
    win_count: usize,
    winning_value: u32,
}

fn get_line_score(line: &str) -> LineResult {
    let mut winning_value = 0;
    let line = &line[line.find(':').unwrap() + 2..];
    let (lhs, mut rhs) = line.split_at(line.find('|').unwrap());
    rhs = &rhs[1..];
    let winning_set: BTreeSet<u32> = lhs
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let my_set: BTreeSet<u32> = rhs
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let win_count = my_set.intersection(&winning_set).count();
    if win_count > 0 {
        winning_value += 2_u32.pow(win_count as u32 - 1);
    }
    LineResult {
        win_count,
        winning_value,
    }
}
