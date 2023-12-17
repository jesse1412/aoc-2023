pub fn part1() -> u32 {
    let mut input = String::from(include_str!(r"inputs\day15.txt"));
    input.retain(|c| c != '\n' && c != '\r');
    let inputs = input.split(',');
    let mut result = 0;
    for s in inputs {
        result += get_instruction_hash(s);
    }
    result
}

pub fn part2() -> u32 {
    let mut input = String::from(include_str!(r"inputs\day15.txt"));
    input.retain(|c| c != '\n' && c != '\r');
    let inputs = input.split(',');
    let instructions: Vec<Instruction> = inputs
        .map(|s| {
            let split = s.split_once('=');
            if let Some((l, r)) = split {
                Instruction::Add(l.to_string(), r.parse().unwrap())
            } else {
                Instruction::Remove(s[0..s.len() - 1].to_string())
            }
        })
        .collect();
    let mut boxes: Vec<Vec<(String, u32)>> = Default::default();
    for _ in 0..=256 {
        boxes.push(Vec::new());
    }
    for instruction in instructions {
        match instruction {
            Instruction::Add(key, val) => {
                let box_vec = &mut boxes[get_instruction_hash(&key) as usize];
                if let Some(entry) = box_vec.iter_mut().find(|(k, _)| *k == key) {
                    *entry = (key, val);
                } else {
                    box_vec.push((key, val));
                }
            }
            Instruction::Remove(key) => {
                let box_vec = &mut boxes[get_instruction_hash(&key) as usize];
                box_vec.retain(|(k, _)| *k != key);
            }
        }
    }
    let mut result = 0;
    for (vi, v) in boxes.iter().enumerate() {
        for (bi, b) in v.iter().enumerate() {
            result += b.1 * (vi + 1) as u32 * (bi + 1) as u32;
        }
    }
    result
}

fn get_instruction_hash(s: &str) -> u32 {
    let mut hash = 0;
    for c in s.chars() {
        hash = get_next_step_hash(hash, &c);
    }
    hash
}

fn get_next_step_hash(prev_hash: u32, next_char: &char) -> u32 {
    let ascii_code = *next_char as u32;
    let hash = prev_hash + ascii_code;
    let hash = hash * 17;
    hash % 256
}

enum Instruction {
    Add(String, u32),
    Remove(String),
}
