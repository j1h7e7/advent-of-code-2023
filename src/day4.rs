use std::collections::HashSet;

fn get_numbers(text: &str) -> HashSet<i32> {
    return text
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}

fn get_card_matches(card: &str) -> usize {
    let trimmed_card: &str = card.split(':').nth(1).unwrap().trim();

    let mut numbers = trimmed_card.split('|').map(|x| get_numbers(x.trim()));

    let have_numbers: HashSet<i32> = numbers.next().unwrap();
    let winning_numbers: HashSet<i32> = numbers.next().unwrap();

    let matches = have_numbers.intersection(&winning_numbers).count();
    return matches;
}

fn get_total_cards(document: &str) -> i32 {
    let mut counts: Vec<i32> = document.lines().map(|_| 1).collect();

    for (i, card) in document.lines().enumerate() {
        let wins = get_card_matches(card);
        for j in 1..wins + 1 {
            counts[i + j] += counts[i];
        }
    }

    return counts.iter().sum::<i32>() as i32;
}

pub struct Day4Puzzle {}
impl super::solve::Puzzle<i32> for Day4Puzzle {
    fn solve(&self, document: &str) -> i32 {
        return document
            .lines()
            .map(|line| match get_card_matches(line) {
                0 => 0,
                n @ _ => 2_i32.pow(n as u32 - 1),
            })
            .sum();
    }

    fn solve2(&self, document: &str) -> i32 {
        return get_total_cards(document);
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
        assert_eq!(get_card_matches("Card 1: 1 2 3 | 2"), 1)
    }
}
