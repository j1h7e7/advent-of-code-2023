use solve::Puzzle;

mod day1;
mod solve;

fn main() {
    let day1_puzzle = day1::Day1Puzzle {};
    let document = solve::read_puzzle_input(day1::Day1Puzzle::DAY);
    println!("{}", day1_puzzle.solve(&document));
}
