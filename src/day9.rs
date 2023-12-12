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

fn get_next_in_sequence_from_line(line: &str) -> i64 {
    let seq: Vec<i64> = line
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();
    return get_next_in_sequence(seq);
}

pub struct Day9Puzzle {}
impl super::solve::Puzzle<String> for Day9Puzzle {
    fn solve(&self, document: &str) -> String {
        return document
            .lines()
            .map(|x| get_next_in_sequence_from_line(x))
            .sum::<i64>()
            .to_string();
    }

    fn solve2(&self, document: &str) -> String {
        panic!("Not implemented");
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
}
