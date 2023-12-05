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

fn split_almanac(almanac: &str) -> (Vec<i64>, std::str::Split<'_, &str>) {
    let mut chunks = almanac.split("\n\n");

    let values = chunks
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    return (values, chunks);
}

fn find_final_values(document: &str) -> Vec<i64> {
    let (mut values, chunks) = split_almanac(document);

    for chunk in chunks {
        let maps = read_maps(chunk);
        values = values
            .iter()
            .map(|x| convert_through_maps(&maps, *x))
            .collect::<Vec<i64>>();
    }

    return values;
}

fn convert_range_through_maps(maps: &Vec<(i64, i64, i64)>, range: (i64, i64)) -> Vec<(i64, i64)> {
    let start = range.0;
    let end = range.1;
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut covered_ranges: Vec<(i64, i64)> = Vec::new();

    for map in maps {
        // if end < map.0 {
        //     continue;
        // }
        // if start > map.0 + map.2 {
        //     break;
        // }
        let conversion = map.1 - map.0;
        let low = map.0.max(start);
        let high = (map.0 + map.2).min(end);
        if low >= high {
            continue;
        }
        ranges.push((low + conversion, high + conversion));
        covered_ranges.push((low, high));
    }
    covered_ranges.sort();

    let mut extra_ranges: Vec<(i64, i64)> = Vec::new();
    let mut active = start;
    for (low, high) in &covered_ranges {
        if low > &active {
            extra_ranges.push((active, *low));
        }
        active = *high;
    }
    if active < end {
        extra_ranges.push((active, end));
    }

    ranges.append(&mut extra_ranges);
    ranges.sort();

    return ranges;
}

fn find_final_ranges(document: &str) -> Vec<(i64, i64)> {
    let (values, chunks) = split_almanac(document);

    let mut ranges = values
        .chunks(2)
        .map(|x| (x[0], x[0] + x[1]))
        .collect::<Vec<(i64, i64)>>();

    for chunk in chunks {
        let maps = read_maps(chunk);
        let list_ranges = ranges
            .iter()
            .map(|x| convert_range_through_maps(&maps, *x))
            .collect::<Vec<Vec<(i64, i64)>>>();
        ranges = list_ranges
            .into_iter()
            .flatten()
            .collect::<Vec<(i64, i64)>>();
    }

    ranges.sort();
    return ranges;
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
        return find_final_ranges(document)[0].0.to_string();
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

    #[test]
    fn test_convert_range_through_maps() {
        assert_eq!(
            convert_range_through_maps(&vec![(10, 20, 10), (20, 50, 10)], (0, 30)),
            vec![(0, 10), (20, 30), (50, 60)]
        );
    }
}
