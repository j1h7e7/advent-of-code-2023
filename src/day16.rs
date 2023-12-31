use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn get_next_position(position: ((i32, i32), Direction), env: char) -> Vec<((i32, i32), Direction)> {
    let ((x, y), direction) = position;

    let n_move = ((x - 1, y), Direction::N);
    let e_move = ((x, y + 1), Direction::E);
    let s_move = ((x + 1, y), Direction::S);
    let w_move = ((x, y - 1), Direction::W);

    if env == '.' {
        return match direction {
            Direction::N => vec![n_move],
            Direction::E => vec![e_move],
            Direction::S => vec![s_move],
            Direction::W => vec![w_move],
        };
    } else if env == '\\' {
        return match direction {
            Direction::N => vec![w_move],
            Direction::E => vec![s_move],
            Direction::S => vec![e_move],
            Direction::W => vec![n_move],
        };
    } else if env == '/' {
        return match direction {
            Direction::N => vec![e_move],
            Direction::E => vec![n_move],
            Direction::S => vec![w_move],
            Direction::W => vec![s_move],
        };
    } else if env == '-' {
        return match direction {
            Direction::N => vec![w_move, e_move],
            Direction::E => vec![e_move],
            Direction::S => vec![e_move, w_move],
            Direction::W => vec![w_move],
        };
    } else if env == '|' {
        return match direction {
            Direction::N => vec![n_move],
            Direction::E => vec![n_move, s_move],
            Direction::S => vec![s_move],
            Direction::W => vec![n_move, s_move],
        };
    }
    panic!("Invalid environment");
}

fn get_energized_tiles(document: &str, start: ((i32, i32), Direction)) -> usize {
    let board = document
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let m = board.len();
    let n = board[0].len();

    let mut positions = vec![start];
    let mut visited: HashSet<((i32, i32), Direction)> = HashSet::new();

    while positions.len() > 0 {
        let mut next_positions = Vec::new();
        for position in positions {
            let ((x, y), _) = position;
            if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                continue;
            }

            if visited.contains(&position) {
                continue;
            }
            visited.insert(position);

            let env = board[x as usize][y as usize];
            let mut next_positions_for_position = get_next_position(position, env);
            next_positions.append(&mut next_positions_for_position);
        }
        positions = next_positions;
    }
    return visited
        .iter()
        .map(|x| x.0)
        .collect::<HashSet<(i32, i32)>>()
        .len();
}

pub struct Day16Puzzle {}
impl super::solve::Puzzle<String> for Day16Puzzle {
    fn solve(&self, document: &str) -> String {
        return get_energized_tiles(document, ((0, 0), Direction::E)).to_string();
    }

    fn solve2(&self, document: &str) -> String {
        let m = document.lines().count() as i32;
        let n = document.lines().next().unwrap().chars().count() as i32;

        let mut ans = 0;
        for i in 0..m {
            ans = ans.max(get_energized_tiles(document, ((i, 0), Direction::E)));
            ans = ans.max(get_energized_tiles(document, ((i, n - 1), Direction::W)));
        }
        for j in 0..n {
            ans = ans.max(get_energized_tiles(document, ((0, j), Direction::S)));
            ans = ans.max(get_energized_tiles(document, ((m - 1, j), Direction::N)));
        }

        return ans.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_position() {
        assert_eq!(
            get_next_position(((0, 0), Direction::E), '.'),
            vec![((0, 1), Direction::E)]
        );
        assert_eq!(
            get_next_position(((0, 0), Direction::E), '\\'),
            vec![((1, 0), Direction::S)]
        );
        assert_eq!(
            get_next_position(((0, 0), Direction::E), '/'),
            vec![((-1, 0), Direction::N)]
        );
        assert_eq!(
            get_next_position(((0, 0), Direction::N), '-'),
            vec![((0, -1), Direction::W), ((0, 1), Direction::E)]
        );
    }

    #[test]
    fn test_get_energized_tiles() {
        let document = "|.\n\\.";
        let energized_tiles = get_energized_tiles(document, ((0, 0), Direction::E));
        assert_eq!(energized_tiles, 3);
    }
}
