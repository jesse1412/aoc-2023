pub fn part1() -> u32 {
    let input = include_str!("inputs/day1.txt");
    let mut res = 0;
    for line in input.lines() {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        for c in line.chars() {
            if c.is_numeric() && first.is_none() {
                first = Some(c);
                last = first;
            } else if c.is_numeric() {
                last = Some(c);
            }
        }
        let v =
            str::parse::<u32>(&(first.unwrap().to_string() + &last.unwrap().to_string())).unwrap();
        res += v;
    }
    res
}

pub fn part2() -> u32 {
    let input = include_str!("inputs/day1.txt").to_string();
    let mut res = 0;
    for line in input.lines() {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        for (i, mut c) in line.chars().enumerate() {
            if line[i..].starts_with("zero") {
                c = '0';
            } else if line[i..].starts_with("one") {
                c = '1';
            } else if line[i..].starts_with("two") {
                c = '2';
            } else if line[i..].starts_with("three") {
                c = '3';
            } else if line[i..].starts_with("four") {
                c = '4';
            } else if line[i..].starts_with("five") {
                c = '5';
            } else if line[i..].starts_with("six") {
                c = '6';
            } else if line[i..].starts_with("seven") {
                c = '7';
            } else if line[i..].starts_with("eight") {
                c = '8';
            } else if line[i..].starts_with("nine") {
                c = '9';
            }
            if c.is_numeric() && first.is_none() {
                first = Some(c);
                last = first;
            } else if c.is_numeric() {
                last = Some(c);
            }
        }
        let v =
            str::parse::<u32>(&(first.unwrap().to_string() + &last.unwrap().to_string())).unwrap();
        res += v;
    }
    res
}
