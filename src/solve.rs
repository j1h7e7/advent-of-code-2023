use std::io::Read;

pub fn read_puzzle_input(day: &str) -> String {
    let mut document: String = String::new();
    let filename: String = format!("input/{}.txt", day);
    std::fs::File::open(filename)
        .expect("Failed to open file")
        .read_to_string(&mut document)
        .expect("Failed to read file");

    return document;
}

pub trait Puzzle<T> {
    fn solve(&self, document: &str) -> T;
    fn solve2(&self, document: &str) -> T;
}
