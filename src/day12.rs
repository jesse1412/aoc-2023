use std::cmp::Ordering;

pub fn part1() -> usize {
    let input = include_str!(r"inputs\day12.txt");
    let input_rows = parse_input(input);
    calculate_possible_solution_count(&input_rows)
}

fn calculate_possible_solution_count(input_rows: &[InputRow]) -> usize {
    let mut all_possible_counts = 0;
    for row in input_rows.iter() {
        // println!("A");
        all_possible_counts += dp(row.input_str, &row.group_sizes, 0);
    }
    all_possible_counts
}

fn dp(remaining_input: &str, remaining_groups: &[usize], curr_hash_streak: usize) -> usize {
    // Base cases, if there are no more groups we hit the end.
    // This situation gives 1 possibility.
    if remaining_groups.is_empty() {
        return 1;
    }
    // Second base case, we're out of input to try.
    // The case has failed, return 0;
    if remaining_input.is_empty() {
        return 0;
    }
    let curr_char = remaining_input.chars().next().unwrap();
    let next_char = remaining_input.chars().nth(1).unwrap_or('.');
    // println!("{curr_char}, {next_char}");
    if curr_char == '#' {
        match (curr_hash_streak + 1).cmp(&remaining_groups[0]) {
            Ordering::Greater => 0,
            Ordering::Equal => {
                if next_char == '#' {
                    0
                } else {
                    dp(
                        remaining_input.get(2..).unwrap_or(""),
                        &remaining_groups[1..],
                        0,
                    )
                }
            }
            Ordering::Less => dp(
                &remaining_input[1..],
                remaining_groups,
                curr_hash_streak + 1,
            ),
        }
    } else if curr_char == '.' {
        if curr_hash_streak > 0 {
            return 0;
        } else {
            return dp(&remaining_input[1..], remaining_groups, 0);
        }
    } else {
        let assume_hash = match (curr_hash_streak + 1).cmp(&remaining_groups[0]) {
            Ordering::Greater => 0,
            Ordering::Equal => {
                if next_char == '#' {
                    0
                } else {
                    dp(
                        remaining_input.get(2..).unwrap_or(""),
                        &remaining_groups[1..],
                        0,
                    )
                }
            }
            Ordering::Less => dp(
                &remaining_input[1..],
                remaining_groups,
                curr_hash_streak + 1,
            ),
        };
        let assume_dot = if curr_hash_streak > 0 {
            0
        } else {
            dp(&remaining_input[1..], remaining_groups, 0)
        };
        return assume_dot + assume_hash;
    }
}

fn parse_input(input: &str) -> Vec<InputRow> {
    let input_rows: Vec<InputRow> = input
        .lines()
        .map(|s| {
            let (input_chars, group_sizes) = s.split_once(' ').unwrap();
            InputRow {
                input_str: input_chars,
                group_sizes: group_sizes
                    .split(',')
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect(),
            }
        })
        .collect();
    input_rows
}

// fn dp();

struct InputRow<'row_life> {
    input_str: &'row_life str,
    group_sizes: Vec<usize>,
}
