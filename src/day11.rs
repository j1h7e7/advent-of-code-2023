fn sum_abs_diff(arr: Vec<i32>) -> i32 {
    let mut a = arr.clone();
    let n = a.len() as i32;
    a.sort_unstable();
    return a
        .iter()
        .enumerate()
        .map(|(i, x)| x * ((2 * i as i32) + 1 - n))
        .sum();
}

fn get_galaxy_positions(document: &str) -> Vec<(i32, i32)> {
    let mut positions = Vec::new();
    let mut rows: Vec<i32> = document.lines().map(|_| 1).collect();
    let mut cols: Vec<i32> = document
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|_| 1)
        .collect();

    for (i, line) in document.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                positions.push((i as i32, j as i32));
                rows[i] = 0;
                cols[j] = 0;
            }
        }
    }

    let row_gaps: Vec<i32> = rows
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();
    let col_gaps: Vec<i32> = cols
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();

    return positions
        .iter()
        .map(|&(i, j)| (i + row_gaps[i as usize], j + col_gaps[j as usize]))
        .collect();
}

pub struct Day11Puzzle {}
impl super::solve::Puzzle<String> for Day11Puzzle {
    fn solve(&self, document: &str) -> String {
        let positions = get_galaxy_positions(document);
        let x: Vec<i32> = positions.iter().map(|&(i, _)| i).collect();
        let y: Vec<i32> = positions.iter().map(|&(_, j)| j).collect();
        return (sum_abs_diff(x) + sum_abs_diff(y)).to_string();
    }

    fn solve2(&self, document: &str) -> String {
        panic!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_abs_diff() {
        assert_eq!(sum_abs_diff(vec![1, 2, 3]), 4);
        assert_eq!(sum_abs_diff(vec![1, 2, 3, 4]), 10);
        assert_eq!(sum_abs_diff(vec![0, 0, 0, 4]), 12);
    }

    #[test]
    fn test_get_galaxy_positions() {
        assert_eq!(get_galaxy_positions(".#.#"), vec![(0, 2), (0, 5)]);
        assert_eq!(get_galaxy_positions(".\n#"), vec![(2, 0)]);
    }
}
