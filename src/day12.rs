use cached::proc_macro::cached;
use std::cmp::Ordering;

pub fn part1() -> usize {
    let input = include_str!(r"inputs\day12.txt");
    let input_rows = parse_input(input, 1);
    calculate_possible_solution_count(&input_rows)
}

pub fn part2() -> usize {
    let input = include_str!(r"inputs\day12.txt");
    let input_rows = parse_input(input, 5);
    calculate_possible_solution_count(&input_rows)
}

fn calculate_possible_solution_count(input_rows: &[InputRow]) -> usize {
    let mut all_possible_counts = 0;
    for row in input_rows.iter() {
        all_possible_counts += dp(row.input_str.clone(), row.group_sizes.clone(), 0);
    }
    all_possible_counts
}

#[cached]
fn dp(remaining_input: String, remaining_groups: Vec<usize>, curr_hash_streak: usize) -> usize {
    // Base cases, if there are no more groups we hit the end.
    if remaining_groups.is_empty() {
        if remaining_input.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }
    // Second base case, we're out of input to try.
    // The case has failed, return 0;
    if remaining_input.is_empty() {
        return 0;
    }

    let curr_char = remaining_input.chars().next().unwrap();
    let next_char = remaining_input.chars().nth(1).unwrap_or('.');
    if curr_char == '#' {
        match (curr_hash_streak + 1).cmp(&remaining_groups[0]) {
            Ordering::Greater => 0,
            Ordering::Equal => {
                if next_char == '#' {
                    0
                } else {
                    dp(
                        String::from(remaining_input.get(2..).unwrap_or("")),
                        remaining_groups[1..].into(),
                        0,
                    )
                }
            }
            Ordering::Less => dp(
                remaining_input[1..].into(),
                remaining_groups,
                curr_hash_streak + 1,
            ),
        }
    } else if curr_char == '.' {
        if curr_hash_streak > 0 {
            return 0;
        } else {
            return dp(remaining_input[1..].into(), remaining_groups, 0);
        }
    } else {
        let assume_hash = match (curr_hash_streak + 1).cmp(&remaining_groups[0]) {
            Ordering::Greater => 0,
            Ordering::Equal => {
                if next_char == '#' {
                    0
                } else {
                    dp(
                        remaining_input.get(2..).unwrap_or("").into(),
                        remaining_groups[1..].into(),
                        0,
                    )
                }
            }
            Ordering::Less => dp(
                remaining_input[1..].into(),
                remaining_groups.clone(),
                curr_hash_streak + 1,
            ),
        };
        let assume_dot = if curr_hash_streak > 0 {
            0
        } else {
            dp(remaining_input[1..].into(), remaining_groups, 0)
        };
        return assume_dot + assume_hash;
    }
}

fn parse_input(input: &str, repeat_count: usize) -> Vec<InputRow> {
    let input_rows: Vec<InputRow> = input
        .lines()
        .map(|s| {
            let (input_chars, group_sizes) = s.split_once(' ').unwrap();
            let mut repeated_input_chars = String::new();
            for _ in 0..repeat_count {
                repeated_input_chars.push_str(input_chars);
                repeated_input_chars.push('?')
            }
            repeated_input_chars.pop();
            InputRow {
                input_str: repeated_input_chars,
                group_sizes: group_sizes
                    .split(',')
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
                    .repeat(repeat_count),
            }
        })
        .collect();
    input_rows
}

struct InputRow {
    input_str: String,
    group_sizes: Vec<usize>,
}
