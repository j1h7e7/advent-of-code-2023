fn get_row_pattern(terrain: &str) -> Vec<u128> {
    return terrain
        .lines()
        .map(|line| {
            let mut ans = 0_u128;
            for c in line.chars() {
                ans <<= 1;
                if c == '#' {
                    ans += 1;
                }
            }
            return ans;
        })
        .collect();
}

fn get_col_pattern(terrain: &str) -> Vec<u128> {
    let mut ans = vec![0_u128; terrain.lines().nth(0).unwrap().len()];
    for line in terrain.lines() {
        for (i, c) in line.chars().enumerate() {
            ans[i] <<= 1;
            if c == '#' {
                ans[i] += 1;
            }
        }
    }
    return ans;
}

fn get_reflection_line(pattern: Vec<u128>) -> usize {
    'main_loop: for i in 1..pattern.len() {
        let iter1 = pattern.iter().take(i).rev();
        let iter2 = pattern.iter().skip(i);
        for (a, b) in iter1.zip(iter2) {
            if a != b {
                continue 'main_loop;
            }
        }
        return i;
    }
    return 0;
}

pub struct Day13Puzzle {}
impl super::solve::Puzzle<String> for Day13Puzzle {
    fn solve(&self, document: &str) -> String {
        return document
            .split("\n\n")
            .map(|terrain| {
                let row_pattern = get_row_pattern(terrain);
                let col_pattern = get_col_pattern(terrain);
                let row_reflection_line = get_reflection_line(row_pattern);
                let col_reflection_line = get_reflection_line(col_pattern);
                return col_reflection_line + 100 * row_reflection_line;
            })
            .sum::<usize>()
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
    fn test_get_patterns() {
        assert_eq!(get_row_pattern("###\n..#\n.#."), vec![7, 1, 2]);
        assert_eq!(get_col_pattern("###\n..#\n.#."), vec![4, 5, 6]);
    }

    #[test]
    fn test_get_reflection_line() {
        assert_eq!(get_reflection_line(vec![1, 1, 2]), 1);
        assert_eq!(get_reflection_line(vec![1, 2, 2, 1]), 2);
        assert_eq!(get_reflection_line(vec![1, 2, 3, 3]), 3);
        assert_eq!(get_reflection_line(vec![1, 2, 3, 4]), 0);
    }
}
