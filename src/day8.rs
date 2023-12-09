use num::integer::lcm;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part1() -> i64 {
    let input = include_str!(r"inputs\day8.txt");
    let lr_iter = get_left_right_iter(input);
    let mappings = get_mappings(input);
    // println!("{:?}", mappings);
    let curr_node = "AAA";
    get_moves_to_z(curr_node, &mappings, lr_iter)
}

fn get_moves_to_z<'a>(
    mut curr_node: &'a str,
    mappings: &'a HashMap<String, (String, String)>,
    mut lr_iter: std::iter::Cycle<std::str::Chars<'_>>,
) -> i64 {
    let mut moves = 0;
    while curr_node != "ZZZ" {
        moves += 1;
        let lr = &mappings[curr_node];
        if lr_iter.next().unwrap() == 'L' {
            curr_node = &lr.0;
        } else {
            curr_node = &lr.1;
        }
    }
    moves
}

pub fn part2() -> i64 {
    let input = include_str!(r"inputs\day8.txt");
    let lr_iter = get_left_right_iter(input);
    let mappings = get_mappings(input);
    // println!("{:?}", mappings);
    let mut curr_nodes: HashSet<&str> = HashSet::new();
    for k in mappings.keys() {
        if k.ends_with('A') {
            curr_nodes.insert(k);
        }
    }
    let moves = curr_nodes
        .iter()
        .map(|s| get_moves_to_z_2(s, &mappings, lr_iter.clone()))
        .fold(1, lcm);
    moves
}

fn get_moves_to_z_2<'a>(
    mut curr_node: &'a str,
    mappings: &'a HashMap<String, (String, String)>,
    mut lr_iter: std::iter::Cycle<std::str::Chars<'_>>,
) -> i64 {
    let mut moves = 0;
    while !curr_node.ends_with('Z') {
        moves += 1;
        let lr = &mappings[curr_node];
        if lr_iter.next().unwrap() == 'L' {
            curr_node = &lr.0;
        } else {
            curr_node = &lr.1;
        }
    }
    moves
}

fn get_mappings(input: &str) -> HashMap<String, (String, String)> {
    let instructions = input.replace(' ', "");
    let mut instruction_lines = instructions.lines();
    instruction_lines.nth(1);
    let mappings: HashMap<String, (String, String)> = instruction_lines
        .map(|s| {
            let split_eq = s.split_once('=').unwrap();
            let (l, r) = split_eq.1[1..split_eq.1.len() - 1].split_once(',').unwrap();
            (split_eq.0.to_string(), (l.to_string(), r.to_string()))
        })
        .collect();
    mappings
}

fn get_left_right_iter(input: &str) -> std::iter::Cycle<std::str::Chars<'_>> {
    let lr = input.lines().next().unwrap();
    let lr_iter = lr.chars().cycle();
    lr_iter
}
