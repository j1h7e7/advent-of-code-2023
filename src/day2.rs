use regex::Regex;

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

#[derive(PartialEq, Debug)]
struct BagReveal {
    red: i32,
    green: i32,
    blue: i32,
}

fn get_reveal(reveal_line: &str) -> BagReveal {
    let mut red: i32 = 0;
    let mut green: i32 = 0;
    let mut blue: i32 = 0;

    let re = Regex::new(r"([0-9]+) (blue|red|green)").unwrap();

    for color in reveal_line.trim().split(',') {
        let (_, [color_value, color_name]) = re.captures(color).unwrap().extract();
        let color_value: i32 = color_value.parse().unwrap();

        match color_name {
            "red" => red += color_value,
            "green" => green += color_value,
            "blue" => blue += color_value,
            _ => (),
        }
    }

    return BagReveal { red, green, blue };
}

fn is_reveal_valid(reveal: &BagReveal) -> bool {
    return reveal.red <= RED && reveal.green <= GREEN && reveal.blue <= BLUE;
}

fn is_game_valid(game_line: &str) -> bool {
    return game_line
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(';')
        .all(|reveal| {
            let reveal = get_reveal(reveal);
            is_reveal_valid(&reveal)
        });
}

fn sum_game_ids(games: &str) -> i32 {
    let re = Regex::new(r"Game ([0-9]+):").unwrap();

    return games
        .lines()
        .filter(|line| is_game_valid(line))
        .map(|line| {
            re.captures(line).unwrap().extract::<1>().1[0]
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>();
}

fn get_game_power(game_line: &str) -> i32 {
    let mut red: i32 = 0;
    let mut green: i32 = 0;
    let mut blue: i32 = 0;

    game_line.split(';').for_each(|reveal| {
        let reveal = get_reveal(reveal);
        red = red.max(reveal.red);
        green = green.max(reveal.green);
        blue = blue.max(reveal.blue);
    });

    return red * green * blue;
}

fn sum_game_power(games: &str) -> i32 {
    return games
        .lines()
        .map(|line| line.split(':').nth(1).unwrap().trim())
        .map(|line| get_game_power(line))
        .sum::<i32>();
}

pub struct Day2Puzzle {}
impl super::solve::Puzzle<i32> for Day2Puzzle {
    fn solve(&self, document: &str) -> i32 {
        return sum_game_ids(document);
    }

    fn solve2(&self, document: &str) -> i32 {
        return sum_game_power(document);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_reveal() {
        assert_eq!(
            get_reveal("1 red, 1 green, 1 blue"),
            BagReveal {
                red: 1,
                green: 1,
                blue: 1
            }
        );
        assert_eq!(
            get_reveal("1 red"),
            BagReveal {
                red: 1,
                green: 0,
                blue: 0
            }
        );
        assert_eq!(
            get_reveal("10 red, 3 green"),
            BagReveal {
                red: 10,
                green: 3,
                blue: 0
            }
        );
    }

    #[test]
    fn test_is_reveal_valid() {
        assert_eq!(
            is_reveal_valid(&BagReveal {
                red: 10,
                green: 10,
                blue: 10
            }),
            true
        );
        assert_eq!(
            is_reveal_valid(&BagReveal {
                red: 10,
                green: 10,
                blue: 15
            }),
            false
        );
    }

    #[test]
    fn test_is_game_valid() {
        assert!(is_game_valid(
            "Game 1: 1 red, 1 green, 1 blue; 1 red, 1 blue"
        ));
        assert!(!is_game_valid(
            "Game 2: 1 red, 1 green, 1 blue; 20 red, 1 green"
        ));
    }

    #[test]
    fn test_sum_game_ids() {
        let document: &str = "Game 1: 1 red, 1 green, 1 blue; 1 red, 1 blue\nGame 2: 1 red, 1 green, 1 blue; 20 red, 1 green";
        assert_eq!(sum_game_ids(document), 1);

        let document: &str = "Game 1: 1 red, 1 green, 1 blue; 1 red, 1 blue\nGame 2: 1 red, 1 green, 1 blue; 1 red, 1 blue";
        assert_eq!(sum_game_ids(document), 3);
    }

    #[test]
    fn test_get_game_power() {
        assert_eq!(
            get_game_power("1 red, 1 green, 1 blue; 1 red, 1 blue"),
            1 * 1 * 1
        );
        assert_eq!(
            get_game_power("1 red, 3 green, 1 blue; 20 red, 1 green"),
            20 * 3 * 1
        );
    }
}
