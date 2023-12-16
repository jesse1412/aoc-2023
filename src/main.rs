mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day1_2015;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    println!("day 1 (2015) part 1: {}", day1_2015::part1());
    println!("day 1 (2015) part 2: {}", day1_2015::part2());

    println!("day 1 part 1: {}", day1::part1());
    println!("day 1 part 2: {}", day1::part2());

    println!("day 2 part 1: {}", day2::part1());
    println!("day 2 part 2: {}", day2::part2());

    println!("day 3 part 1: {}", day3::part1());
    println!("day 3 part 2: {}", day3::part2());

    println!("day 4 part 1: {}", day4::part1());
    println!("day 4 part 2: {}", day4::part2());

    println!("day 5 part 1: {}", day5::part1());
    println!("day 5 part 2: {}", day5::part2());

    println!("day 6 part 1: {}", day6::part1());
    println!("day 6 part 2: {}", day6::part2());

    println!("day 7 part 1: {}", day7::part1());
    println!("day 7 part 2: {}", day7::part2());

    println!("day 8 part 1: {}", day8::part1());
    println!("day 8 part 2: {}", day8::part2());

    println!("day 9 part 1: {}", day9::part1());
    println!("day 9 part 2: {}", day9::part2());

    println!("day 10 part 1: {}", day10::part1());
    println!("day 10 part 2: {}", day10::part2());

    println!("day 11 part 1: {}", day11::day11(2));
    println!("day 11 part 2: {}", day11::day11(1000000));

    // Really unhappy with day 12.
    // Had to use a lot of clone and passing whole strings...
    // Current run time is >1s...
    // println!("day 12 part 1: {}", day12::part1());
    // println!("day 12 part 2: {}", day12::part2());

    println!("day 13 part 1: {}", day13::part1());
    println!("day 13 part 2: {}", day13::part2());

    // Again, unhappy with day 12.
    // Had to use a lot of data duplication.
    // Current run time is >1s...
    // println!("day 14 part 1: {}", day14::part1());
    // println!("day 14 part 2: {}", day14::part2(1000000000));

    println!("day 15 part 1: {}", day15::part1());
    println!("day 15 part 2: {}", day15::part2());
}
