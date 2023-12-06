use itertools::Itertools;

fn extract_numbers_from_line(line: &str) -> Vec<i32> {
    return line
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn get_time_range(time_limit: i32, distance: i32) -> (i32, i32) {
    // distance if time is T and time held is t, is t*(T-t)
    // so we require t*(T-t) > D
    // wolfram alpha gives
    let disc = (time_limit * time_limit) - (4 * distance);
    let root_disc = (disc as f64).sqrt();
    let lower = ((time_limit as f64) - root_disc) / 2.0;
    let upper = ((time_limit as f64) + root_disc) / 2.0;
    return (lower.floor() as i32 + 1, upper.ceil() as i32 - 1);
}

pub struct Day6Puzzle {}
impl super::solve::Puzzle<String> for Day6Puzzle {
    fn solve(&self, document: &str) -> String {
        let (times, distances) = document
            .lines()
            .map(|line| extract_numbers_from_line(line))
            .collect_tuple()
            .unwrap();

        let mut ans = 1;
        for (time, distance) in times.iter().zip(distances.iter()) {
            let (lower, upper) = get_time_range(*time, *distance);
            ans *= upper - lower + 1;
        }
        return ans.to_string();
    }

    fn solve2(&self, document: &str) -> String {
        panic!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers_from_line() {
        assert_eq!(extract_numbers_from_line("Time: 1  2  3"), vec![1, 2, 3]);
    }

    #[test]
    fn test_get_time_range() {
        assert_eq!(get_time_range(10, 10), (2, 8));
    }
}
