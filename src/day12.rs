use std::cmp::Ordering;
use std::collections::HashMap;

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
        let mut dynamic_program: DynamicProgram = DynamicProgram {
            input_str: row.input_str.clone(),
            groups: row.group_sizes.clone(),
            cache: HashMap::new(),
        };
        all_possible_counts += dynamic_program.dp(0, 0, 0);
    }
    all_possible_counts
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

#[derive(Default)]
struct DynamicProgram {
    input_str: String,
    groups: Vec<usize>,
    cache: HashMap<(usize, usize, usize), usize>,
}

impl DynamicProgram {
    fn dp(
        &mut self,
        curr_input_pos: usize,
        curr_group_pos: usize,
        curr_hash_streak: usize,
    ) -> usize {
        let hash_struct = (curr_input_pos, curr_group_pos, curr_hash_streak);
        if let Some(v) = self.cache.get(&hash_struct) {
            return *v;
        }
        let remaining_input = self.get_remaining_input(curr_input_pos);
        let remaining_groups = self.get_remaining_groups(curr_group_pos);

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
                        self.cache.insert(hash_struct, 0);
                        0
                    } else {
                        let res = self.dp(curr_input_pos + 2, curr_group_pos + 1, 0);
                        self.cache.insert(hash_struct, res);
                        res
                    }
                }
                Ordering::Less => {
                    let res = self.dp(curr_input_pos + 1, curr_group_pos, curr_hash_streak + 1);
                    self.cache.insert(hash_struct, res);
                    res
                }
            }
        } else if curr_char == '.' {
            if curr_hash_streak > 0 {
                self.cache.insert(hash_struct, 0);
                0
            } else {
                let res = self.dp(curr_input_pos + 1, curr_group_pos, 0);
                self.cache.insert(hash_struct, res);
                res
            }
        } else {
            let assume_hash = match (curr_hash_streak + 1).cmp(&remaining_groups[0]) {
                Ordering::Greater => 0,
                Ordering::Equal => {
                    if next_char == '#' {
                        0
                    } else {
                        self.dp(curr_input_pos + 2, curr_group_pos + 1, 0)
                    }
                }
                Ordering::Less => self.dp(curr_input_pos + 1, curr_group_pos, curr_hash_streak + 1),
            };
            let assume_dot = if curr_hash_streak > 0 {
                0
            } else {
                self.dp(curr_input_pos + 1, curr_group_pos, 0)
            };
            self.cache.insert(hash_struct, assume_dot + assume_hash);
            return assume_dot + assume_hash;
        }
    }

    fn get_remaining_groups(&self, curr_group_pos: usize) -> &[usize] {
        let remaining_groups: &[usize] = if curr_group_pos < self.groups.len() {
            &self.groups[curr_group_pos..]
        } else {
            &[]
        };
        remaining_groups
    }

    fn get_remaining_input(&self, curr_input_pos: usize) -> &str {
        let remaining_input: &str = if curr_input_pos < self.input_str.len() {
            &self.input_str[curr_input_pos..]
        } else {
            ""
        };
        remaining_input
    }
}
