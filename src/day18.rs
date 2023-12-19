use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn get_line_instruction(line: &str) -> (usize, Direction) {
    let dir = match line.chars().nth(0).unwrap() {
        'U' => Direction::N,
        'R' => Direction::E,
        'D' => Direction::S,
        'L' => Direction::W,
        _ => panic!("Invalid direction"),
    };
    let steps = line.split(' ').nth(1).unwrap().parse::<usize>().unwrap();
    return (steps, dir);
}

fn get_instructions(document: &str) -> Vec<(usize, Direction)> {
    return document
        .lines()
        .map(|line| get_line_instruction(line))
        .collect();
}

fn get_point_sequence(instructions: Vec<(usize, Direction)>) -> Vec<(isize, isize)> {
    let mut points: Vec<(isize, isize)> = Vec::new();
    let mut point = (0, 0);
    points.push(point);
    for (steps, direction) in instructions {
        match direction {
            Direction::N => point.1 -= steps as isize,
            Direction::E => point.0 += steps as isize,
            Direction::S => point.1 += steps as isize,
            Direction::W => point.0 -= steps as isize,
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
        let instructions = get_instructions(document);

        let bondary_size = instructions.iter().map(|(steps, _)| steps).sum::<usize>() / 2;

        let points = get_point_sequence(instructions);
        let area = shoelace_area(&points) + bondary_size as isize + 1;
        return area.to_string();
    }

    fn solve2(&self, document: &str) -> String {
        panic!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line_instruction() {
        assert_eq!(get_line_instruction("D 10 (#6cc0d3)"), (10, Direction::S));
    }
}
