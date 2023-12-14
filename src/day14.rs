fn get_lane_load(lane: &str) -> u32 {
    let mut load: u32 = 0;
    let mut slot: u32 = 0;
    for (i, c) in lane.chars().enumerate() {
        match c {
            'O' => {
                load += lane.len() as u32 - slot;
                slot += 1;
            }
            '#' => slot = i as u32 + 1,
            _ => (),
        }
    }
    return load;
}

pub struct Day14Puzzle {}
impl super::solve::Puzzle<String> for Day14Puzzle {
    fn solve(&self, document: &str) -> String {
        let line_len = document.lines().next().unwrap().len();
        let lanes = (0..line_len)
            .map(|i| {
                document
                    .lines()
                    .map(|line| line.chars().nth(i).unwrap())
                    .collect::<String>()
            })
            .collect::<Vec<String>>();
        return lanes
            .iter()
            .map(|lane| get_lane_load(lane))
            .sum::<u32>()
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
    fn test_get_lane_load() {
        assert_eq!(get_lane_load("..."), 0);
        assert_eq!(get_lane_load("O.."), 3);
        assert_eq!(get_lane_load("..O"), 3);
        assert_eq!(get_lane_load(".#O"), 1);
        assert_eq!(get_lane_load("O.O"), 5);
    }
}
