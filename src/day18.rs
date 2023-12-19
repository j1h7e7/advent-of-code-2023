use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn get_line_instruction(line: &str) -> (usize, Direction, &str) {
    let dir = match line.chars().nth(0).unwrap() {
        'U' => Direction::N,
        'R' => Direction::E,
        'D' => Direction::S,
        'L' => Direction::W,
        _ => panic!("Invalid direction"),
    };
    let steps = line.split(' ').nth(1).unwrap().parse::<usize>().unwrap();
    let unparsed_color: &str = line.split(' ').nth(2).unwrap();
    return (steps, dir, unparsed_color[2..].trim_end_matches(')'));
}

fn get_instructions(document: &str) -> Vec<(usize, Direction, &str)> {
    return document
        .lines()
        .map(|line| get_line_instruction(line))
        .collect();
}

fn draw_line(
    canvas: &mut HashSet<(isize, isize)>,
    origin: (isize, isize),
    direction: Direction,
    steps: isize,
) -> (isize, isize) {
    let (mut x, mut y) = origin;
    for _ in 0..steps {
        match direction {
            Direction::N => y -= 1,
            Direction::E => x += 1,
            Direction::S => y += 1,
            Direction::W => x -= 1,
        }
        canvas.insert((x, y));
    }
    return (x, y);
}

fn follow_instructions(document: &str) -> HashSet<(isize, isize)> {
    let instructions = get_instructions(document);
    let mut canvas: HashSet<(isize, isize)> = HashSet::new();
    let mut origin = (0, 0);
    canvas.insert(origin);
    for (steps, direction, _color) in instructions {
        origin = draw_line(&mut canvas, origin, direction, steps as isize);
    }
    return canvas;
}

fn flood_fill(canvas: HashSet<(isize, isize)>, origin: (isize, isize)) -> HashSet<(isize, isize)> {
    let mut visited: HashSet<(isize, isize)> = canvas.clone();
    let mut queue: Vec<(isize, isize)> = Vec::new();
    queue.push(origin);
    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        queue.push((x - 1, y));
        queue.push((x + 1, y));
        queue.push((x, y - 1));
        queue.push((x, y + 1));
    }
    return visited;
}

pub struct Day18Puzzle {}
impl super::solve::Puzzle<String> for Day18Puzzle {
    fn solve(&self, document: &str) -> String {
        let canvas = follow_instructions(document);
        let filled = flood_fill(canvas, (1, 1));
        return filled.len().to_string();
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
        assert_eq!(
            get_line_instruction("D 10 (#6cc0d3)"),
            (10, Direction::S, "6cc0d3")
        );
    }

    #[test]
    fn test_draw_line() {
        let mut canvas: HashSet<(isize, isize)> = HashSet::new();
        draw_line(&mut canvas, (0, 0), Direction::N, 3);
        assert!(canvas.contains(&(0, -1)));
        assert!(canvas.contains(&(0, -2)));
        assert!(canvas.contains(&(0, -3)));
        draw_line(&mut canvas, (10, 10), Direction::E, 3);
        assert!(canvas.contains(&(11, 10)));
        assert!(canvas.contains(&(12, 10)));
        assert!(canvas.contains(&(13, 10)));
    }
}
