use std::collections::HashMap;

fn get_single_node(node_info: &str) -> (&str, &str, &str) {
    let node_name: &str = &node_info[0..3];
    let left_node_name: &str = &node_info[7..10];
    let right_node_name: &str = &node_info[12..15];
    return (node_name, left_node_name, right_node_name);
}

fn get_network(document: &str) -> HashMap<&str, (&str, &str)> {
    let mut raw_node_map: HashMap<&str, (&str, &str)> = HashMap::new();
    for node_info in document.lines().skip(2) {
        let (node_name, left_node_name, right_node_name) = get_single_node(node_info);
        raw_node_map.insert(node_name, (left_node_name, right_node_name));
    }
    return raw_node_map;
}

#[derive(Debug, PartialEq)]
enum Direction {
    L,
    R,
}

fn take_step<'a>(
    current_node: &'a str,
    node_map: &HashMap<&'a str, (&'a str, &'a str)>,
    direction: Direction,
) -> &'a str {
    let (left_node_name, right_node_name) = node_map.get(current_node).unwrap();
    match direction {
        Direction::L => return *left_node_name,
        Direction::R => return *right_node_name,
    }
}

fn get_direction_sequence<'a>(route: &'a str) -> impl Iterator<Item = (usize, Direction)> + 'a {
    return route
        .chars()
        .map(|x| match x {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("Invalid direction"),
        })
        .clone()
        .cycle()
        .enumerate();
}

pub struct Day8Puzzle {}
impl super::solve::Puzzle<String> for Day8Puzzle {
    fn solve(&self, document: &str) -> String {
        let directions = get_direction_sequence(document.lines().nth(0).unwrap());
        let node_map: HashMap<&str, (&str, &str)> = get_network(document);

        let mut current_node = "AAA";
        for (step, direction) in directions {
            current_node = take_step(current_node, &node_map, direction);
            if current_node == "ZZZ" {
                return format!("{}", step + 1);
            }
        }
        panic!("Unreachable");
    }

    fn solve2(&self, document: &str) -> String {
        panic!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_single_node() {
        let node_info: &str = "AAA = (BBB, CCC)";
        let (node_name, left_node_name, right_node_name) = get_single_node(node_info);
        assert_eq!(node_name, "AAA");
        assert_eq!(left_node_name, "BBB");
        assert_eq!(right_node_name, "CCC");
    }

    #[test]
    fn test_step() {
        let current_node = "AAA";
        let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();
        node_map.insert("AAA", ("BBB", "CCC"));

        assert_eq!(take_step(current_node, &node_map, Direction::L), "BBB");
    }

    #[test]
    fn test_get_direction_sequence() {
        let route = "LRL";
        let mut direction_sequence = get_direction_sequence(route);
        assert_eq!(direction_sequence.next().unwrap(), (0, Direction::L));
        assert_eq!(direction_sequence.next().unwrap(), (1, Direction::R));
        assert_eq!(direction_sequence.next().unwrap(), (2, Direction::L));
        assert_eq!(direction_sequence.next().unwrap(), (3, Direction::L));
        assert_eq!(direction_sequence.next().unwrap(), (4, Direction::R));
        assert_eq!(direction_sequence.next().unwrap(), (5, Direction::L));
    }
}
