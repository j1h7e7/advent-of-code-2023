use std::collections::HashMap;

fn get_lane_load(lane: &str) -> u32 {
    let mut load: u32 = 0;
    let mut slot: u32 = 0;
    for (i, c) in lane.chars().enumerate() {
        match c {
            'O' => {
                load += lane.len() as u32 - slot;
                slot += 1;
            }
            '#' => slot = i as u32 + 1,
            _ => (),
        }
    }
    return load;
}

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

fn transpose(document: &str) -> String {
    let line_len = document.lines().next().unwrap().len();
    let lanes = (0..line_len)
        .map(|i| {
            document
                .lines()
                .map(|line| line.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect::<Vec<String>>();
    return lanes.join("\n");
}
fn flip(document: &str) -> String {
    return document
        .lines()
        .map(|line| line.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}
fn tilt_row_left(row: &str) -> String {
    let mut new_row = String::new();

    let mut slot: usize = 0;
    for (i, c) in row.chars().enumerate() {
        match c {
            'O' => {
                new_row.push('O');
                slot += 1;
            }
            '#' => {
                for _ in 0..i - slot {
                    new_row.push('.');
                }
                new_row.push('#');
                slot = i + 1;
            }
            '.' => (),
            _ => panic!("Invalid character"),
        }
    }
    for _ in 0..row.len() - slot as usize {
        new_row.push('.');
    }

    return new_row;
}
fn tilt_board(board: &str) -> String {
    return board
        .lines()
        .map(|line| tilt_row_left(line))
        .collect::<Vec<String>>()
        .join("\n");
}
fn spin_cycle(board: &str) -> String {
    let mut board = board.to_string(); // start: NESW
    board = transpose(&board); // WSEN
    board = tilt_board(&board); // N
    board = transpose(&board); // NESW
    board = tilt_board(&board); // W
    board = transpose(&board); // WSEN
    board = flip(&board); // WNES
    board = tilt_board(&board); // S
    board = transpose(&board); // SENW
    board = flip(&board); // SWNE
    board = tilt_board(&board); // E
    board = flip(&board); // SENW
    board = transpose(&board); // WNES
    board = flip(&board); // WSEN
    board = transpose(&board); // NESW

    return board;
}

pub struct Day14Puzzle {}
impl super::solve::Puzzle<String> for Day14Puzzle {
    fn solve(&self, document: &str) -> String {
        let line_len = document.lines().next().unwrap().len();
        let lanes = (0..line_len)
            .map(|i| {
                document
                    .lines()
                    .map(|line| line.chars().nth(i).unwrap())
                    .collect::<String>()
            })
            .collect::<Vec<String>>();
        return lanes
            .iter()
            .map(|lane| get_lane_load(lane))
            .sum::<u32>()
            .to_string();
    }

    fn solve2(&self, document: &str) -> String {
        let mut states: HashMap<String, usize> = HashMap::new();
        let mut board = document.to_string();
        let mut i = 0;
        loop {
            if states.contains_key(&board) {
                break;
            }
            states.insert(board.clone(), i);
            board = spin_cycle(&board);
            i += 1;
        }
        let cycle_start = states.get(&board).unwrap();
        let cycle_length = i - cycle_start;
        let cycle_index = (1000000000 - cycle_start) % cycle_length;

        let board = states
            .iter()
            .find(|(_, v)| **v == cycle_index + cycle_start)
            .unwrap()
            .0;

        return transpose(board)
            .lines()
            .map(|lane| get_simplified_lane_load(lane))
            .sum::<u32>()
            .to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lane_load() {
        assert_eq!(get_lane_load("..."), 0);
        assert_eq!(get_lane_load("O.."), 3);
        assert_eq!(get_lane_load("..O"), 3);
        assert_eq!(get_lane_load(".#O"), 1);
        assert_eq!(get_lane_load("O.O"), 5);
    }

    #[test]
    fn test_get_simple_lane_load() {
        assert_eq!(get_simplified_lane_load("..."), 0);
        assert_eq!(get_simplified_lane_load("O.."), 3);
        assert_eq!(get_simplified_lane_load("..O"), 1);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(transpose("..."), ".\n.\n.");
        assert_eq!(transpose("...\n..."), "..\n..\n..");
        assert_eq!(transpose(".\n.\n."), "...");
    }

    #[test]
    fn test_flip() {
        assert_eq!(flip("..#"), "#..");
        assert_eq!(flip("..#\n#.."), "#..\n..#");
    }

    #[test]
    fn test_tilt_row() {
        assert_eq!(tilt_row_left("..O"), "O..");
        assert_eq!(tilt_row_left(".#.O"), ".#O.");
        assert_eq!(tilt_row_left("..O...O"), "OO.....");
    }
}
