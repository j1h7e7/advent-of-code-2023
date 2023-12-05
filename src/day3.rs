use std::collections::{HashMap, HashSet};

fn get_symbol_positions(schematic: &str) -> Vec<Vec<bool>> {
    let h: usize = schematic.lines().count();
    let l: usize = schematic.lines().nth(0).unwrap().chars().count();

    let mut positions = vec![vec![false; l]; h];

    for (i, line) in schematic.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) || c == '.' {
                continue;
            }
            for x in i.checked_sub(1).unwrap_or(0)..h.min(i + 2) {
                for y in j.checked_sub(1).unwrap_or(0)..l.min(j + 2) {
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

fn get_potential_gears(schematic: &str) -> Vec<Vec<Vec<(usize, usize)>>> {
    let h: usize = schematic.lines().count();
    let l: usize = schematic.lines().nth(0).unwrap().chars().count();

    let mut positions: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![Vec::new(); l]; h];

    for (i, line) in schematic.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '*' {
                continue;
            }
            for x in i.checked_sub(1).unwrap_or(0)..h.min(i + 2) {
                for y in j.checked_sub(1).unwrap_or(0)..l.min(j + 2) {
                    positions[x][y].push((i, j));
                }
            }
        }
    }

    return positions;
}

fn get_gear_values(schematic: &str) -> HashMap<(usize, usize), Vec<i32>> {
    let positions = get_potential_gears(schematic);

    let mut gear_values: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let mut buffer: String = String::new();
    let mut active_gears: HashSet<(usize, usize)> = HashSet::new();

    fn end_num(
        buffer: &mut String,
        active_gears: &mut HashSet<(usize, usize)>,
        gear_values: &mut HashMap<(usize, usize), Vec<i32>>,
    ) -> () {
        if buffer.len() == 0 {
            return;
        }
        let num = buffer.parse::<i32>().unwrap();
        for gear in active_gears.iter() {
            gear_values.entry(*gear).or_insert(Vec::new()).push(num);
        }
        buffer.clear();
        active_gears.clear();
    }

    for (i, line) in schematic.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                buffer.push(c);
                active_gears.extend(positions[i][j].clone());
            } else {
                end_num(&mut buffer, &mut active_gears, &mut gear_values);
            }
        }
        end_num(&mut buffer, &mut active_gears, &mut gear_values);
    }
    end_num(&mut buffer, &mut active_gears, &mut gear_values);
    return gear_values;
}

fn sum_gear_ratios(gear_values: &HashMap<(usize, usize), Vec<i32>>) -> i32 {
    let mut sum: i32 = 0;
    for (_, values) in gear_values.iter() {
        if values.len() != 2 {
            continue;
        }
        sum += values[0] * values[1];
    }
    return sum;
}

pub struct Day3Puzzle {}
impl super::solve::Puzzle<String> for Day3Puzzle {
    fn solve(&self, document: &str) -> String {
        let positions = get_symbol_positions(document);
        return document
            .lines()
            .zip(positions.iter())
            .map(|(line, pos_row)| sum_line_numbers(line, pos_row))
            .sum::<i32>()
            .to_string();
    }

    fn solve2(&self, document: &str) -> String {
        return sum_gear_ratios(&get_gear_values(document)).to_string();
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

    #[test]
    fn test_get_potential_gears() {
        assert_eq!(
            get_potential_gears(".*."),
            vec![vec![vec![(0, 1)], vec![(0, 1)], vec![(0, 1)]]]
        );
        assert_eq!(
            get_potential_gears("*.*"),
            vec![vec![vec![(0, 0)], vec![(0, 0), (0, 2)], vec![(0, 2)]]]
        );
    }

    #[test]
    fn test_get_gear_values() {
        assert_eq!(
            get_gear_values("1*2\n35."),
            HashMap::from_iter(vec![((0, 1), vec![1, 2, 35])])
        );
    }

    #[test]
    fn test_sum_gear_ratios() {
        assert_eq!(
            sum_gear_ratios(&HashMap::from_iter(vec![
                ((0, 1), vec![1, 2]),
                ((0, 2), vec![3, 4, 5]),
                ((1, 1), vec![6]),
            ])),
            2
        );
    }
}
