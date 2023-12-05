use itertools::Itertools;

fn read_single_map(text: &str) -> (i64, i64, i64) {
    let tuple: (i64, i64, i64) = text
        .split_whitespace()
        .map(|line| line.parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();
    return (tuple.1, tuple.0, tuple.2);
}

fn read_maps(text: &str) -> Vec<(i64, i64, i64)> {
    let mut maps = text
        .lines()
        .skip(1)
        .map(|line| read_single_map(line))
        .collect::<Vec<(i64, i64, i64)>>();
    maps.sort_by(|a, b| a.0.cmp(&b.0));
    return maps;
}

fn convert_through_maps(maps: &Vec<(i64, i64, i64)>, value: i64) -> i64 {
    let idx = maps
        .binary_search(&(value, i64::MAX, i64::MAX))
        .unwrap_or_else(|x| x);

    if idx == 0 {
        return value;
    }
    let map = maps[idx - 1];
    if value < map.0 + map.2 {
        return value + (map.1 - map.0);
    }
    return value;
}

fn find_final_values(document: &str) -> Vec<i64> {
    let mut chunks = document.split("\n\n");

    let mut values = chunks
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for chunk in chunks {
        let maps = read_maps(chunk);
        values = values
            .iter()
            .map(|x| convert_through_maps(&maps, *x))
            .collect::<Vec<i64>>();
    }

    return values;
}

pub struct Day5Puzzle {}
impl super::solve::Puzzle<String> for Day5Puzzle {
    fn solve(&self, document: &str) -> String {
        return find_final_values(document)
            .iter()
            .min()
            .unwrap()
            .to_string();
    }

    fn solve2(&self, document: &str) -> String {
        return String::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_single_map() {
        assert_eq!(read_single_map(" 1  2  3 "), (2, 1, 3));
    }

    #[test]
    fn test_convert_through_maps() {
        assert_eq!(convert_through_maps(&vec![(50, 98, 2)], 50), 98);
        assert_eq!(convert_through_maps(&vec![(50, 98, 2)], 51), 99);
        assert_eq!(convert_through_maps(&vec![(50, 98, 2)], 10), 10);
        assert_eq!(convert_through_maps(&vec![(1, 3, 2), (5, 10, 2)], 6), 11);
        assert_eq!(convert_through_maps(&vec![(1, 3, 2), (5, 10, 2)], 3), 3);
    }
}
