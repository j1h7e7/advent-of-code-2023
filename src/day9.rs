use itertools::Itertools;

fn get_next_in_sequence(seq: Vec<i64>) -> i64 {
    let mut ans = Vec::new();
    let mut cur_seq = seq;

    while !cur_seq.iter().all(|x| *x == 0) {
        ans.push(cur_seq[cur_seq.len() - 1]);
        let next_seq = cur_seq
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();
        cur_seq = next_seq;
    }

    return ans.iter().sum();
}

fn get_previous_in_sequence(seq: Vec<i64>) -> i64 {
    let mut ans = Vec::new();
    let mut cur_seq = seq;

    while !cur_seq.iter().all(|x| *x == 0) {
        ans.push(cur_seq[0]);
        let next_seq = cur_seq
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();
        cur_seq = next_seq;
    }

    return ans
        .iter()
        .enumerate()
        .map(|(i, x)| {
            x * match i % 2 {
                0 => 1,
                1 => -1,
                _ => panic!("Invalid index"),
            }
        })
        .sum();
}

pub struct Day9Puzzle {}
impl super::solve::Puzzle<String> for Day9Puzzle {
    fn solve(&self, document: &str) -> String {
        return document
            .lines()
            .map(|x| {
                x.split_whitespace()
                    .map(|y| y.parse::<i64>().unwrap())
                    .collect_vec()
            })
            .map(|x| get_next_in_sequence(x))
            .sum::<i64>()
            .to_string();
    }

    fn solve2(&self, document: &str) -> String {
        return document
            .lines()
            .map(|x| {
                x.split_whitespace()
                    .map(|y| y.parse::<i64>().unwrap())
                    .collect_vec()
            })
            .map(|x| get_previous_in_sequence(x))
            .sum::<i64>()
            .to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_in_sequence() {
        assert_eq!(get_next_in_sequence(vec![1, 1, 1, 1]), 1);
        assert_eq!(get_next_in_sequence(vec![1, 3, 5, 7]), 9);
        assert_eq!(get_next_in_sequence(vec![1, 3, 6, 10]), 15);
    }

    #[test]
    fn test_get_previous_in_sequence() {
        assert_eq!(get_previous_in_sequence(vec![1, 1, 1, 1]), 1);
        assert_eq!(get_previous_in_sequence(vec![3, 5, 7, 9]), 1);
        assert_eq!(get_previous_in_sequence(vec![3, 6, 10, 15]), 1);
    }
}
