use std::collections::HashSet;

fn get_numbers(text: &str) -> HashSet<i32> {
    return text
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}

fn get_card_score(card: &str) -> i32 {
    let trimmed_card: &str = card.split(':').nth(1).unwrap().trim();

    let mut numbers = trimmed_card.split('|').map(|x| get_numbers(x.trim()));

    let have_numbers: HashSet<i32> = numbers.next().unwrap();
    let winning_numbers: HashSet<i32> = numbers.next().unwrap();

    let matches = have_numbers.intersection(&winning_numbers).count();
    return match matches {
        0 => 0,
        _ => (2 as i32).pow((matches - 1).try_into().unwrap()),
    };
}

pub struct Day4Puzzle {}
impl super::solve::Puzzle<i32> for Day4Puzzle {
    fn solve(&self, document: &str) -> i32 {
        return document.lines().map(|line| get_card_score(line)).sum();
    }

    fn solve2(&self, document: &str) -> i32 {
        panic!("Not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_numbers() {
        assert_eq!(get_numbers("1 2 3"), HashSet::from([1, 2, 3]));
        assert_eq!(
            get_numbers(" 1  2  3 40  5"),
            HashSet::from([1, 2, 3, 40, 5])
        );
    }

    #[test]
    fn test_get_score() {
        assert_eq!(get_card_score("Card 1: 1 2 3 | 2"), 1)
    }
}
