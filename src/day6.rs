use itertools::Itertools;

fn extract_numbers_from_line(line: &str) -> Vec<i64> {
    return line
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
}

fn get_time_range(time_limit: i64, distance: i64) -> (i64, i64) {
    // distance if time is T and time held is t, is t*(T-t)
    // so we require t*(T-t) > D
    // wolfram alpha gives
    let disc = (time_limit * time_limit) - (4 * distance);
    let root_disc = (disc as f64).sqrt();
    let lower = ((time_limit as f64) - root_disc) / 2.0;
    let upper = ((time_limit as f64) + root_disc) / 2.0;
    return (lower.floor() as i64 + 1, upper.ceil() as i64 - 1);
}

fn extract_long_number_from_line(line: &str) -> i64 {
    return line
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
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
        let (time, distance) = document
            .lines()
            .map(|line| extract_long_number_from_line(line))
            .collect_tuple()
            .unwrap();

        let ans = get_time_range(time, distance);
        return (ans.1 - ans.0 + 1).to_string();
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

    #[test]
    fn test_extract_long_number_from_line() {
        assert_eq!(extract_long_number_from_line("Time: 1  2  3"), 123);
    }
}
