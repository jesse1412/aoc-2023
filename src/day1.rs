pub fn part1() -> i32 {
    let input = include_str!("inputs/day1.txt");
    let mut floor: i32 = 0;
    input.chars().for_each(|c| match c {
        '(' => floor += 1,
        ')' => floor -= 1,
        _ => (),
    });
    floor
}

pub fn part2() -> usize {
    let input = include_str!("inputs/day1.txt");
    let mut floor: i32 = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        };
        if floor < 0 {
            return i + 1;
        }
    }
    0
}
