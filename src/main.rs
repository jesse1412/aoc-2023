mod day1;
mod day10;
mod day11;
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
}
