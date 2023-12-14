use std::collections::HashMap;

fn get_simplified_lane_load(lane: &str) -> u32 {
    return lane
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            'O' => lane.len() as u32 - i as u32,
            _ => 0,
        })
        .sum();
}

fn transpose(document: &mut Vec<String>) -> () {
    // assume document is square
    *document = (0..document.len())
        .map(|i| {
            document
                .iter()
                .map(|line| line.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect::<Vec<String>>();
}
fn flip(document: &mut Vec<String>) -> () {
    *document = document
        .iter()
        .map(|line| line.chars().rev().collect::<String>())
        .collect::<Vec<String>>();
}
fn tilt_row_left(row: &mut String) -> () {
    let old_row = row.clone();
    row.clear();

    let mut slot: usize = 0;
    for (i, c) in old_row.chars().enumerate() {
        match c {
            'O' => {
                row.push('O');
                slot += 1;
            }
            '#' => {
                for _ in 0..i - slot {
                    row.push('.');
                }
                row.push('#');
                slot = i + 1;
            }
            '.' => (),
            _ => panic!("Invalid character"),
        }
    }
    for _ in 0..old_row.len() - slot as usize {
        row.push('.');
    }
}
fn tilt_board(board: &mut Vec<String>) -> () {
    for row in board {
        tilt_row_left(row);
    }
}
fn spin_cycle(board: &mut Vec<String>) -> () {
    // start: NESW
    transpose(board); // WSEN
    tilt_board(board); // N
    transpose(board); // NESW
    tilt_board(board); // W
    transpose(board); // WSEN
    flip(board); // WNES
    tilt_board(board); // S
    transpose(board); // SENW
    flip(board); // SWNE
    tilt_board(board); // E
    flip(board); // SENW
    transpose(board); // WNES
    flip(board); // WSEN
    transpose(board); // NESW
}

pub struct Day14Puzzle {}
impl super::solve::Puzzle<String> for Day14Puzzle {
    fn solve(&self, document: &str) -> String {
        let mut board: Vec<String> = document
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();
        transpose(&mut board);
        tilt_board(&mut board);
        return board
            .iter()
            .map(|lane| get_simplified_lane_load(lane))
            .sum::<u32>()
            .to_string();
    }

    fn solve2(&self, document: &str) -> String {
        let mut states: HashMap<Vec<String>, usize> = HashMap::new();
        let mut board: Vec<String> = document
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();
        let mut i = 0;
        loop {
            if states.contains_key(&board) {
                break;
            }
            states.insert(board.clone(), i);
            spin_cycle(&mut board);
            i += 1;
        }
        let cycle_start = states.get(&board).unwrap();
        let cycle_length = i - cycle_start;
        let cycle_index = (1000000000 - cycle_start) % cycle_length;

        let mut board = states
            .iter()
            .find(|(_, v)| **v == cycle_index + cycle_start)
            .unwrap()
            .0
            .clone();

        transpose(&mut board);

        return board
            .iter()
            .map(|lane| get_simplified_lane_load(lane))
            .sum::<u32>()
            .to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_simple_lane_load() {
        assert_eq!(get_simplified_lane_load("..."), 0);
        assert_eq!(get_simplified_lane_load("O.."), 3);
        assert_eq!(get_simplified_lane_load("..O"), 1);
    }

    #[test]
    fn test_transpose() {
        let mut vec: Vec<String> = vec![String::from("123"); 3];
        transpose(&mut vec);
        assert_eq!(
            vec,
            vec![
                String::from("111"),
                String::from("222"),
                String::from("333")
            ]
        );
    }

    #[test]
    fn test_flip() {
        let mut vec: Vec<String> = vec![String::from("#."), String::from(".#")];
        flip(&mut vec);
        assert_eq!(vec, vec![String::from(".#"), String::from("#.")]);
    }

    #[test]
    fn test_tilt_row() {
        let mut row = String::from("O..#..O.O...O..#O");
        tilt_row_left(&mut row);
        assert_eq!(row, String::from("O..#OOO........#O"));
    }
}
