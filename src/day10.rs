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

pub struct Day10Puzzle {}
impl super::solve::Puzzle<String> for Day10Puzzle {
    fn solve(&self, document: &str) -> String {
        let (nodes, s_cell) = get_nodes(document);
        return get_farthest_distance(&nodes, s_cell).to_string();
    }

    fn solve2(&self, document: &str) -> String {
        panic!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nodes() {
        let document = "S-7\n|.|\nL-J";
        let (nodes, _) = get_nodes(document);
        assert_eq!(nodes.len(), 8);
        assert_eq!(nodes[&(0, 0)], ((1, 0), (0, 1)));
        assert_eq!(nodes[&(1, 0)], ((0, 0), (2, 0)));
    }
}
