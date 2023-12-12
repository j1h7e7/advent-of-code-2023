use std::collections::{HashMap, HashSet};

use phf::phf_map;

static PIPES: phf::Map<&'static str, ((i32, i32), (i32, i32))> = phf_map! {
    "-" => ((-1,0),(1,0)),
    "|" => ((0,-1),(0,1)),
    "L" => ((0,-1),(1,0)),
    "J" => ((0,-1),(-1,0)),
    "7" => ((0,1),(-1,0)),
    "F" => ((0,1),(1,0)),
};

fn get_nodes(document: &str) -> (HashMap<(i32, i32), ((i32, i32), (i32, i32))>, (i32, i32)) {
    let mut s_cell: (i32, i32) = (-1, -1);
    let mut nodes: HashMap<(i32, i32), ((i32, i32), (i32, i32))> = HashMap::new();
    for (y, line) in document.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if c == 'S' {
                s_cell = (x as i32, y as i32);
                continue;
            }
            let pipe = PIPES[&c.to_string()];
            nodes.insert(
                (x as i32, y as i32),
                (
                    (x as i32 + pipe.0 .0, y as i32 + pipe.0 .1),
                    (x as i32 + pipe.1 .0, y as i32 + pipe.1 .1),
                ),
            );
        }
    }

    let mut s_conns: Vec<(i32, i32)> = Vec::new();
    for (src, (d1, d2)) in nodes.iter() {
        if *d1 == s_cell || *d2 == s_cell {
            s_conns.push(*src);
        }
    }
    nodes.insert(s_cell, (s_conns[0], s_conns[1]));

    return (nodes, s_cell);
}

fn get_farthest_distance(
    nodes: &HashMap<(i32, i32), ((i32, i32), (i32, i32))>,
    start: (i32, i32),
) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: Vec<(i32, i32)> = Vec::new();
    queue.push(start);
    let mut distance = 0;
    'inf: loop {
        let mut next_queue: Vec<(i32, i32)> = Vec::new();
        for node in queue.iter() {
            if visited.contains(node) {
                break 'inf;
            }
            visited.insert(*node);
            let (d1, d2) = nodes[node];
            if !visited.contains(&d1) {
                next_queue.push(d1);
            }
            if !visited.contains(&d2) {
                next_queue.push(d2);
            }
        }
        distance += 1;
        queue = next_queue;
    }
    return distance;
}

fn get_main_path(
    nodes: &HashMap<(i32, i32), ((i32, i32), (i32, i32))>,
    start: (i32, i32),
) -> HashSet<(i32, i32)> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: Vec<(i32, i32)> = Vec::new();
    queue.push(start);
    'inf: loop {
        let mut next_queue: Vec<(i32, i32)> = Vec::new();
        for node in queue.iter() {
            if visited.contains(node) {
                break 'inf;
            }
            visited.insert(*node);
            let (d1, d2) = nodes[node];
            if !visited.contains(&d1) {
                next_queue.push(d1);
            }
            if !visited.contains(&d2) {
                next_queue.push(d2);
            }
        }
        queue = next_queue;
    }
    return visited;
}

fn get_s_type(s_node: (i32, i32), nodes: &HashMap<(i32, i32), ((i32, i32), (i32, i32))>) -> char {
    let (n1, n2) = nodes[&s_node];
    let d1 = (n1.0 - s_node.0, n1.1 - s_node.1);
    let d2 = (n2.0 - s_node.0, n2.1 - s_node.1);
    let diffs: HashSet<(i32, i32)> = HashSet::from_iter(vec![d1, d2]);

    for (c, (d1, d2)) in PIPES.entries() {
        if diffs.contains(d1) && diffs.contains(d2) {
            return c.chars().next().unwrap();
        }
    }
    return ' ';
}

fn get_internal_points(document: &str) -> HashSet<(i32, i32)> {
    let (nodes, s_cell) = get_nodes(document);
    let main_path = get_main_path(&nodes, s_cell);
    let mut internal_points: HashSet<(i32, i32)> = HashSet::new();

    for (y, row) in document.lines().enumerate() {
        let mut is_internal = false;
        let mut last_vert: char = ' ';
        for (x, mut c) in row.chars().enumerate() {
            if c == 'S' {
                c = get_s_type((x as i32, y as i32), &nodes);
            }
            if main_path.contains(&(x as i32, y as i32)) {
                if c == '|' {
                    is_internal = !is_internal;
                } else if c == '-' {
                    continue;
                } else if last_vert == ' ' {
                    last_vert = c;
                } else {
                    let b0 = last_vert == 'L';
                    let b1 = c == 'J';
                    if b0 ^ b1 {
                        is_internal = !is_internal;
                    }
                    last_vert = ' ';
                }
                continue;
            }

            if is_internal {
                internal_points.insert((x as i32, y as i32));
            }
        }
    }
    return internal_points;
}

pub struct Day10Puzzle {}
impl super::solve::Puzzle<String> for Day10Puzzle {
    fn solve(&self, document: &str) -> String {
        let (nodes, s_cell) = get_nodes(document);
        return get_farthest_distance(&nodes, s_cell).to_string();
    }

    fn solve2(&self, document: &str) -> String {
        return get_internal_points(document).len().to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOCUMENT: &str = "S-7\n|.|\nL-J";

    #[test]
    fn test_get_nodes() {
        let (nodes, _) = get_nodes(DOCUMENT);
        assert_eq!(nodes.len(), 8);
        assert!(nodes[&(0, 0)] == ((0, 1), (1, 0)) || nodes[&(0, 0)] == ((1, 0), (0, 1)));
    }

    #[test]
    fn test_get_main_path() {
        let (nodes, s_cell) = get_nodes(DOCUMENT);
        let main_path = get_main_path(&nodes, s_cell);
        assert_eq!(
            main_path,
            HashSet::from_iter(vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (0, 1),
                (2, 1),
                (0, 2),
                (1, 2),
                (2, 2)
            ])
        );
    }

    #[test]
    fn test_get_internal_points() {
        let internal_points = get_internal_points(DOCUMENT);
        assert_eq!(internal_points, HashSet::from_iter(vec![(1, 1)]));
    }

    #[test]
    fn test_get_s_type() {
        let (nodes, _) = get_nodes(DOCUMENT);
        assert_eq!(get_s_type((0, 0), &nodes), 'F');
        assert_eq!(get_s_type((0, 2), &nodes), 'L');
        assert_eq!(get_s_type((2, 0), &nodes), '7');
        assert_eq!(get_s_type((2, 2), &nodes), 'J');
        assert_eq!(get_s_type((1, 0), &nodes), '-');
        assert_eq!(get_s_type((0, 1), &nodes), '|');
    }
}
