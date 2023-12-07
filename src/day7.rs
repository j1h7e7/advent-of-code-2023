use std::collections::HashMap;

use itertools::Itertools;
use phf::phf_map;

static CARDS: phf::Map<&'static str, i32> = phf_map! {
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
    "T" => 10,
    "J" => 11,
    "Q" => 12,
    "K" => 13,
    "A" => 14,
};

fn get_hand_rank(hand: &str) -> (i32, i32, i32, i32, i32, i32) {
    let mut pairs: HashMap<String, i32> = HashMap::new();
    for card in hand.trim().chars() {
        pairs
            .entry(card.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    let mut pair_structure = pairs.values().collect::<Vec<&i32>>();
    pair_structure.sort_unstable();
    pair_structure.reverse();

    let hand_type = match pair_structure.as_slice() {
        [5] => 6,
        [4, 1] => 5,
        [3, 2] => 4,
        [3, 1, 1] => 3,
        [2, 2, 1] => 2,
        [2, 1, 1, 1] => 1,
        [1, 1, 1, 1, 1] => 0,
        _ => panic!("Invalid hand"),
    };

    return (
        hand_type,
        CARDS[&hand[0..1]],
        CARDS[&hand[1..2]],
        CARDS[&hand[2..3]],
        CARDS[&hand[3..4]],
        CARDS[&hand[4..5]],
    );
}

fn get_hand_rank_and_bid(
    line: &str,
    hand_ranker: &dyn Fn(&str) -> (i32, i32, i32, i32, i32, i32),
) -> ((i32, i32, i32, i32, i32, i32), i32) {
    let mut parts = line.trim().split_whitespace();
    let hand = parts.next().unwrap();
    let bid = parts.next().unwrap();
    let hand_rank = hand_ranker(hand);
    let bid = bid.parse::<i32>().unwrap();
    return (hand_rank, bid);
}

fn get_hand_rank_wild(hand: &str) -> (i32, i32, i32, i32, i32, i32) {
    let mut pairs: HashMap<String, i32> = HashMap::new();
    let mut wilds: i32 = 0;
    for card in hand.trim().chars() {
        if card == 'J' {
            wilds += 1;
            continue;
        }

        pairs
            .entry(card.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    let mut pair_structure = pairs.values().collect::<Vec<&i32>>();
    pair_structure.sort_unstable();
    pair_structure.reverse();

    let hand_type = match wilds {
        0 => match pair_structure.as_slice() {
            [5] => 6,
            [4, 1] => 5,
            [3, 2] => 4,
            [3, 1, 1] => 3,
            [2, 2, 1] => 2,
            [2, 1, 1, 1] => 1,
            [1, 1, 1, 1, 1] => 0,
            _ => panic!("Invalid hand"),
        },
        1 => match pair_structure.as_slice() {
            [4] => 6,
            [3, 1] => 5,
            [2, 2] => 4,
            [2, 1, 1] => 3,
            [1, 1, 1, 1] => 1,
            _ => panic!("Invalid hand"),
        },
        2 => match pair_structure.as_slice() {
            [3] => 6,
            [2, 1] => 5,
            [1, 1, 1] => 3,
            _ => panic!("Invalid hand"),
        },
        3 => match pair_structure.as_slice() {
            [2] => 6,
            [1, 1] => 5,
            _ => panic!("Invalid hand"),
        },
        4..=5 => 6,
        _ => panic!("Invalid hand"),
    };

    let mut card_ranks: HashMap<&str, i32> = HashMap::new();
    card_ranks.extend(&CARDS);
    card_ranks.insert("J", 0);

    return (
        hand_type,
        card_ranks[&hand[0..1]],
        card_ranks[&hand[1..2]],
        card_ranks[&hand[2..3]],
        card_ranks[&hand[3..4]],
        card_ranks[&hand[4..5]],
    );
}

pub struct Day7Puzzle {}
impl super::solve::Puzzle<String> for Day7Puzzle {
    fn solve(&self, document: &str) -> String {
        return document
            .lines()
            .map(|x| get_hand_rank_and_bid(x, &get_hand_rank))
            .sorted_unstable()
            .enumerate()
            .map(|(i, (_, bid))| (i as i32 + 1) * bid)
            .sum::<i32>()
            .to_string();
    }

    fn solve2(&self, document: &str) -> String {
        return document
            .lines()
            .map(|x| get_hand_rank_and_bid(x, &get_hand_rank_wild))
            .sorted_unstable()
            .enumerate()
            .map(|(i, (_, bid))| (i as i32 + 1) * bid)
            .sum::<i32>()
            .to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hand_rank() {
        assert_eq!(get_hand_rank("AA3AA"), (5, 14, 14, 3, 14, 14));
        assert_eq!(get_hand_rank("57385"), (1, 5, 7, 3, 8, 5));
        assert_eq!(get_hand_rank("AA3KK"), (2, 14, 14, 3, 13, 13));
    }

    #[test]
    fn test_get_hand_rank_wild() {
        assert_eq!(get_hand_rank_wild("AA3AA"), (5, 14, 14, 3, 14, 14));
        assert_eq!(get_hand_rank_wild("AA3JJ"), (5, 14, 14, 3, 0, 0));
        assert_eq!(get_hand_rank_wild("57J85"), (3, 5, 7, 0, 8, 5));
    }
}
