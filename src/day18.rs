use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    U,
    R,
    D,
    L,
}

fn get_line_instruction(line: &str) -> (u64, Direction) {
    let dir = match line.chars().nth(0).unwrap() {
        'U' => Direction::U,
        'R' => Direction::R,
        'D' => Direction::D,
        'L' => Direction::L,
        _ => panic!("Invalid direction"),
    };
    let steps = line.split(' ').nth(1).unwrap().parse::<u64>().unwrap();
    return (steps, dir);
}

fn get_updated_line_instruction(line: &str) -> (u64, Direction) {
    let color = line.split("#").nth(1).unwrap()[0..6].to_string();

    let distance = u64::from_str_radix(&color[0..5], 16).unwrap();
    let dir = match color.chars().nth(5).unwrap() {
        '0' => Direction::R,
        '1' => Direction::D,
        '2' => Direction::L,
        '3' => Direction::U,
        _ => panic!("Invalid direction"),
    };

    return (distance, dir);
}

fn get_instructions(
    document: &str,
    get_line_instruction: &dyn Fn(&str) -> (u64, Direction),
) -> Vec<(u64, Direction)> {
    return document
        .lines()
        .map(|line| get_line_instruction(line))
        .collect();
}

fn get_point_sequence(instructions: Vec<(u64, Direction)>) -> Vec<(isize, isize)> {
    let mut points: Vec<(isize, isize)> = Vec::new();
    let mut point = (0, 0);
    points.push(point);
    for (steps, direction) in instructions {
        match direction {
            Direction::U => point.1 -= steps as isize,
            Direction::R => point.0 += steps as isize,
            Direction::D => point.1 += steps as isize,
            Direction::L => point.0 -= steps as isize,
        }
        points.push(point);
    }
    return points;
}

fn shoelace_area(points: &Vec<(isize, isize)>) -> isize {
    let mut area = 0;
    for pair in points.windows(2) {
        let (x0, y0) = pair[0];
        let (x1, y1) = pair[1];
        area += (y0 + y1) * (x0 - x1);
    }
    return area.abs() / 2;
}

pub struct Day18Puzzle {}
impl super::solve::Puzzle<String> for Day18Puzzle {
    fn solve(&self, document: &str) -> String {
        let instructions = get_instructions(document, &get_line_instruction);

        let bondary_size = instructions.iter().map(|(steps, _)| steps).sum::<u64>() / 2;

        let points = get_point_sequence(instructions);
        let area = shoelace_area(&points) + bondary_size as isize + 1;
        return area.to_string();
    }

    fn solve2(&self, document: &str) -> String {
        let instructions = get_instructions(document, &get_updated_line_instruction);

        let bondary_size = instructions.iter().map(|(steps, _)| steps).sum::<u64>() / 2;

        let points = get_point_sequence(instructions);
        let area = shoelace_area(&points) + bondary_size as isize + 1;
        return area.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line_instruction() {
        assert_eq!(get_line_instruction("D 10 (#6cc0d3)"), (10, Direction::D));
    }
}
