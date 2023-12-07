use std::collections::HashMap;

pub fn part1() -> u64 {
    let input = include_str!(r"inputs\day5.txt");
    let mut lines = input.lines();
    let mut seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    seeds.sort_unstable();
    let mappings = get_mappings(lines);
    get_mapped_seeds(mappings, &mut seeds);
    // println!("{:?}", mappings);
    *seeds.iter().min().unwrap()
}

// Part 2 logic:
// Step one, sort your seed pairs by start pos.
// Step two, sort your mappings by start pos.
// While current max seed range end < next map start, apply current map offset.
//  If you pass current map end, swap to no map and create a new section.
// Once at next map start:
//  If seed is in range start the while loop again.
//  Else, go to the next map.
// Might be useful to add placeholder range at the end.
// Finally, sum up all ranges with their offsets applied.
pub fn part2() -> u64 {
    let input = include_str!(r"inputs\day5.txt");
    let mut lines = input.lines();
    let mut seeds: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|x| (x[0], x[0] + x[1]))
        .collect();
    seeds.sort_unstable();
    let mut mappings = get_mappings(lines);
    for (_, v) in mappings.iter_mut() {
        v.sort_unstable();
    }
    let mut curr_mapping = "seed";
    while let Some(mappings) = mappings.get(curr_mapping) {
        let mut new_seeds: Vec<(u64, u64)> = Vec::new();
        while let Some((seed_start, seed_end)) = seeds.pop() {
            let mut seed_mapped = false;
            for map in mappings {
                curr_mapping = &map.to_name;
                if seed_start < map.start_pos && seed_end >= map.start_pos {
                    seeds.push((seed_start, map.start_pos - 1));
                    seeds.push((map.start_pos, seed_end));
                    seed_mapped = true;
                } else if seed_start < map.end_pos && seed_end >= map.end_pos {
                    seeds.push((seed_start, map.end_pos - 1));
                    seeds.push((map.end_pos, seed_end));
                    seed_mapped = true;
                } else if seed_start >= map.start_pos && seed_end < map.end_pos {
                    new_seeds.push((
                        (seed_start as i64 + map.offset) as u64,
                        (seed_end as i64 + map.offset) as u64,
                    ));
                    seed_mapped = true;
                    break;
                }
            }
            if !seed_mapped {
                new_seeds.push((seed_start, seed_end));
            }
        }
        seeds = new_seeds;
    }
    // println!("{:?}", mappings);
    seeds.iter().min().unwrap().0
}

fn get_mapped_seeds(mappings: HashMap<String, Vec<Mapping>>, seeds: &mut [u64]) {
    let mut map_name = "seed";
    while map_name != "location" {
        let maps = &mappings[map_name];
        for (i, seed) in (seeds).to_vec().clone().iter().enumerate() {
            for map in maps {
                map_name = &map.to_name;
                if *seed >= map.start_pos && *seed < map.end_pos {
                    seeds[i] = (*seed as i64 + map.offset) as u64;
                }
            }
        }
    }
}

fn get_mappings(mut lines: std::str::Lines<'_>) -> HashMap<String, Vec<Mapping>> {
    let mut mappings: HashMap<String, Vec<Mapping>> = HashMap::new();
    while let Some(s) = lines.next() {
        if s.len() > 1 {
            let (from_name, to_name) = s.split_once(' ').unwrap().0.split_once("-to-").unwrap();
            for s in lines.by_ref() {
                if s.len() <= 1 {
                    break;
                }
                let mut split_line = s.split(' ');
                let to_pos = split_line.next().unwrap().parse::<u64>().unwrap();
                let start_pos = split_line.next().unwrap().parse::<u64>().unwrap();
                let offset = to_pos as i64 - start_pos as i64;
                let end_pos = start_pos + split_line.next().unwrap().parse::<u64>().unwrap();
                mappings
                    .entry(String::from(from_name))
                    .or_default()
                    .push(Mapping {
                        from_name: String::from(from_name),
                        to_name: String::from(to_name),
                        start_pos,
                        end_pos,
                        offset,
                    });
            }
        }
    }
    mappings
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SeedRange {
    start_pos: u64,
    end_pos: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Mapping {
    start_pos: u64,
    from_name: String,
    to_name: String,
    end_pos: u64,
    offset: i64,
}
