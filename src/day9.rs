use std::collections::HashSet;

pub fn part1() -> i64 {
    let input = include_str!(r"inputs\day9.txt");
    let histories = get_histories(input);
    let mut result = 0;
    for hist in histories {
        let all_diffs = compute_all_diff_steps(hist);
        result += all_diffs.iter().fold(0, |acc, v| acc + v.last().unwrap());
    }
    result
}

pub fn part2() -> i64 {
    let input = include_str!(r"inputs\day9.txt");
    let histories = get_histories(input);
    let mut result = 0;
    for mut hist in histories {
        hist.reverse();
        let all_diffs = compute_all_diff_steps(hist);
        let test = all_diffs.iter().fold(0, |acc, v| acc + v.last().unwrap());
        result += test
    }
    result
}

fn compute_all_diff_steps(hist: Vec<i64>) -> Vec<Vec<i64>> {
    let mut diffs_set: HashSet<i64> = HashSet::new();
    let mut all_diffs: Vec<Vec<i64>> = vec![hist.clone()];
    let mut curr_diffs: Vec<i64> = hist;
    while !(diffs_set.contains(&0) && diffs_set.len() == 1) {
        curr_diffs = compute_single_diff_step(&curr_diffs);
        diffs_set = HashSet::from_iter(curr_diffs.clone());
        all_diffs.push(curr_diffs.clone());
    }
    all_diffs
}

fn compute_single_diff_step(curr_diffs: &[i64]) -> Vec<i64> {
    curr_diffs
        .iter()
        .scan(0, |a, b| {
            let tmp = *a;
            *a = *b;
            Some(b - tmp)
        })
        .skip(1)
        .collect()
}

fn get_histories(input: &str) -> Vec<Vec<i64>> {
    let histories: Vec<Vec<i64>> = input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    histories
}
