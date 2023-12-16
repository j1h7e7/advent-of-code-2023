fn hash_string(input: &str) -> i32 {
    return input
        .chars()
        .map(|c| c as u8)
        .fold(0_u8, |acc, x| acc.wrapping_add(x).wrapping_mul(17)) as i32;
}

pub struct Day15Puzzle {}
impl super::solve::Puzzle<String> for Day15Puzzle {
    fn solve(&self, document: &str) -> String {
        return document
            .split(',')
            .map(hash_string)
            .sum::<i32>()
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
    fn test_hash_string() {
        assert_eq!(hash_string("rn=1"), 30);
    }
}
