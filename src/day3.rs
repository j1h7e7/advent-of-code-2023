fn get_symbol_positions(schematic: &str) -> Vec<Vec<bool>> {
    let h: usize = schematic.lines().count();
    let l: usize = schematic.lines().nth(0).unwrap().chars().count();

    let mut positions = vec![vec![false; l]; h];

    for (i, line) in schematic.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) || c == '.' {
                continue;
            }
            for x in 0.max(i - 1)..h.min(i + 2) {
                for y in 0.max(j - 1)..l.min(j + 2) {
                    positions[x][y] = true;
                }
            }
        }
    }

    return positions;
}

fn sum_line_numbers(line: &str, positions: &Vec<bool>) -> i32 {
    let mut sum: i32 = 0;
    let mut buffer: String = String::new();
    let mut active_valid: bool = false;

    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            buffer.push(c);
            active_valid |= positions[i];
        } else {
            if active_valid {
                sum += buffer.parse::<i32>().unwrap();
            }
            buffer = String::new();
            active_valid = false;
        }
    }

    if active_valid {
        sum += buffer.parse::<i32>().unwrap();
    }

    return sum;
}

pub struct Day3Puzzle {}
impl super::solve::Puzzle<i32> for Day3Puzzle {
    fn solve(&self, document: &str) -> i32 {
        let positions = get_symbol_positions(document);
        return document
            .lines()
            .zip(positions.iter())
            .map(|(line, pos_row)| sum_line_numbers(line, pos_row))
            .sum();
    }

    fn solve2(&self, document: &str) -> i32 {
        panic!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_symbol_positions() {
        assert_eq!(
            get_symbol_positions("....\n....\n...#"),
            vec![
                vec![false, false, false, false],
                vec![false, false, true, true],
                vec![false, false, true, true],
            ]
        );
        assert_eq!(
            get_symbol_positions("....\n....\n...#\n...."),
            vec![
                vec![false, false, false, false],
                vec![false, false, true, true],
                vec![false, false, true, true],
                vec![false, false, true, true],
            ]
        );
        assert_eq!(
            get_symbol_positions("...\n.#.\n..."),
            vec![vec![true; 3]; 3]
        )
    }

    #[test]
    fn test_sum_line_numbers() {
        assert_eq!(
            sum_line_numbers("1.2.3", &vec![false, false, false, false, false]),
            0
        );
        assert_eq!(
            sum_line_numbers("1.2.3", &vec![false, true, false, true, false]),
            0
        );
        assert_eq!(
            sum_line_numbers("1.2.3", &vec![true, true, true, true, false]),
            3
        );
        assert_eq!(sum_line_numbers("12.13", &vec![true; 5]), 25);
        assert_eq!(sum_line_numbers("12#13", &vec![true; 5]), 25);
    }
}
