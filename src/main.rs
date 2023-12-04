mod day1;
mod day1_2015;
mod day2;
mod day3;
mod day4;

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
}
