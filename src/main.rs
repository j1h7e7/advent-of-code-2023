use solve::Puzzle;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod solve;

fn main() {
    println!("Enter day number:");
    let mut day = String::new();
    std::io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");
    let day: i32 = day.trim().parse().expect("Please type a number!");

    let puzzle: Box<dyn Puzzle<String>> = match day {
        1 => Box::new(day1::Day1Puzzle {}),
        2 => Box::new(day2::Day2Puzzle {}),
        3 => Box::new(day3::Day3Puzzle {}),
        4 => Box::new(day4::Day4Puzzle {}),
        5 => Box::new(day5::Day5Puzzle {}),
        6 => Box::new(day6::Day6Puzzle {}),
        7 => Box::new(day7::Day7Puzzle {}),
        8 => Box::new(day8::Day8Puzzle {}),
        9 => Box::new(day9::Day9Puzzle {}),
        10 => Box::new(day10::Day10Puzzle {}),
        11 => Box::new(day11::Day11Puzzle {}),
        12 => Box::new(day12::Day12Puzzle {}),
        13 => Box::new(day13::Day13Puzzle {}),
        14 => Box::new(day14::Day14Puzzle {}),
        _ => panic!("Invalid day number"),
    };

    println!("Enter part number:");
    let mut part = String::new();
    std::io::stdin()
        .read_line(&mut part)
        .expect("Failed to read line");
    let part: i32 = part.trim().parse().expect("Please type a number!");

    println!("Run test case? (y/n)");
    let mut test_case = String::new();
    std::io::stdin()
        .read_line(&mut test_case)
        .expect("Failed to read line");
    let test_case: bool = test_case.trim() == "y";

    let document = match test_case {
        true => solve::read_puzzle_input(&format!("day{}test", day)),
        false => solve::read_puzzle_input(&format!("day{}", day)),
    };
    let answer = match part {
        1 => puzzle.solve(&document),
        2 => puzzle.solve2(&document),
        _ => panic!("Invalid part number"),
    };
    println!("{}", answer);
}
