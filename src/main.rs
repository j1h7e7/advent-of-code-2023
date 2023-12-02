use solve::Puzzle;

mod day1;
mod day2;
mod solve;

fn main() {
    println!("Enter day number:");
    let mut day = String::new();
    std::io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");
    let day: i32 = day.trim().parse().expect("Please type a number!");

    let puzzle: Box<dyn Puzzle<i32>> = match day {
        1 => Box::new(day1::Day1Puzzle {}),
        2 => Box::new(day2::Day2Puzzle {}),
        _ => panic!("Invalid day number"),
    };

    println!("Enter part number:");
    let mut part = String::new();
    std::io::stdin()
        .read_line(&mut part)
        .expect("Failed to read line");
    let part: i32 = part.trim().parse().expect("Please type a number!");

    let document = solve::read_puzzle_input(&format!("day{}", day));
    let answer = match part {
        1 => puzzle.solve(&document),
        2 => puzzle.solve2(&document),
        _ => panic!("Invalid part number"),
    };
    println!("{}", answer);
}
