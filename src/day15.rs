use std::collections::HashMap;

fn hash_string(input: &str) -> u8 {
    return input
        .chars()
        .map(|c| c as u8)
        .fold(0_u8, |acc, x| acc.wrapping_add(x).wrapping_mul(17));
}

#[derive(Debug, Clone)]
struct LensBox {
    lenses: Vec<usize>,
    lens_pos: HashMap<String, usize>,
    i: usize,
}

fn execute_lens_command(boxes: &mut Vec<LensBox>, command: &str) -> () {
    match command.chars().last().unwrap() {
        '-' => execute_subtract_command(boxes, command),
        _ => execute_set_command(boxes, command),
    }
}

fn execute_subtract_command(boxes: &mut Vec<LensBox>, command: &str) -> () {
    let name = &command[..command.len() - 1];
    let hash = hash_string(name);
    let active_box = &mut boxes[hash as usize];

    if !active_box.lens_pos.contains_key(name) {
        return;
    }

    let pos = active_box.lens_pos.get(name).unwrap();
    active_box.lenses[*pos] = 0;
    active_box.lens_pos.remove(name);
}

fn execute_set_command(boxes: &mut Vec<LensBox>, command: &str) -> () {
    let name = command.split('=').into_iter().nth(0).unwrap();
    let lens_number = command
        .split('=')
        .into_iter()
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let hash = hash_string(name);
    let active_box = &mut boxes[hash as usize];

    if active_box.lens_pos.contains_key(name) {
        let pos = active_box.lens_pos.get(name).unwrap();
        active_box.lenses[*pos] = lens_number;
    } else {
        active_box.lenses.push(lens_number);
        active_box.lens_pos.insert(name.to_string(), active_box.i);
        active_box.i += 1;
    }
}

fn get_box_power(lens_box: &LensBox) -> i32 {
    return lens_box
        .lenses
        .iter()
        .filter(|x| **x != 0)
        .enumerate()
        .map(|(i, x)| ((i + 1) * x) as i32)
        .sum();
}

pub struct Day15Puzzle {}
impl super::solve::Puzzle<String> for Day15Puzzle {
    fn solve(&self, document: &str) -> String {
        return document
            .split(',')
            .map(|x| hash_string(x) as i32)
            .sum::<i32>()
            .to_string();
    }

    fn solve2(&self, document: &str) -> String {
        let mut boxes = vec![
            LensBox {
                lenses: Vec::new(),
                lens_pos: HashMap::new(),
                i: 0,
            };
            256
        ];

        for command in document.split(',') {
            execute_lens_command(&mut boxes, command);
        }

        return boxes
            .iter()
            .enumerate()
            .map(|(i, x)| (i as i32 + 1) * get_box_power(x))
            .sum::<i32>()
            .to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        assert_eq!(hash_string("rn=1"), 30);
    }

    #[test]
    fn test_execute_commands() {
        let boxes = &mut vec![
            LensBox {
                lenses: Vec::new(),
                lens_pos: HashMap::new(),
                i: 0,
            };
            256
        ];
        execute_lens_command(boxes, "rn=1");
        assert_eq!(boxes[0].lenses, vec![1]);
        assert_eq!(boxes[0].lens_pos.get("rn"), Some(&0));
        execute_lens_command(boxes, "rn=2");
        assert_eq!(boxes[0].lenses, vec![2]);
        assert_eq!(boxes[0].lens_pos.get("rn"), Some(&0));
        execute_lens_command(boxes, "qp=3");
        assert_eq!(boxes[1].lenses, vec![3]);
        assert_eq!(boxes[1].lens_pos.get("qp"), Some(&0));
        execute_lens_command(boxes, "qp-");
        assert_eq!(boxes[1].lenses, vec![0]);
        assert_eq!(boxes[1].lens_pos.get("qp"), None);
        execute_lens_command(boxes, "qp=3");
        assert_eq!(boxes[1].lenses, vec![0, 3]);
        assert_eq!(boxes[1].lens_pos.get("qp"), Some(&1));
    }
}
