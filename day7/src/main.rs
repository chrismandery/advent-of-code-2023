use anyhow::{ensure, Context, Result};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Card {
    Joker,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(c: char) -> Result<Card, Self::Error> {
        match c {
            '2' => Ok(Card::N2),
            '3' => Ok(Card::N3),
            '4' => Ok(Card::N4),
            '5' => Ok(Card::N5),
            '6' => Ok(Card::N6),
            '7' => Ok(Card::N7),
            '8' => Ok(Card::N8),
            '9' => Ok(Card::N9),
            'T' => Ok(Card::T),
            'J' => Ok(Card::J),
            'Q' => Ok(Card::Q),
            'K' => Ok(Card::K),
            'A' => Ok(Card::A),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ht_self = self.calc_hand_type_with_replaced_jokers();
        let ht_other = other.calc_hand_type_with_replaced_jokers();

        let cmp = ht_self.cmp(&ht_other);
        match cmp {
            Ordering::Equal => {
                // Run tie-breaker
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    let cmp2 = c1.cmp(c2);
                    if cmp2 != Ordering::Equal {
                        return cmp2;
                    }
                }

                Ordering::Equal
            }
            _ => cmp,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    /// Calculate best hand type using a brute force search where each joker is replaced by every possible hand.
    fn calc_hand_type_with_replaced_jokers(&self) -> HandType {
        let cards_without_jokers: Vec<Card> = self
            .cards
            .iter()
            .filter(|c| **c != Card::Joker)
            .cloned()
            .collect();
        let joker_count = 5 - cards_without_jokers.len();

        if joker_count > 0 {
            let possible_cards = [
                Card::N2,
                Card::N3,
                Card::N4,
                Card::N5,
                Card::N6,
                Card::N7,
                Card::N8,
                Card::N9,
                Card::T,
                Card::J,
                Card::Q,
                Card::K,
                Card::A,
            ];

            let cards_to_add = (0..joker_count)
                .map(|_| possible_cards.into_iter())
                .multi_cartesian_product();

            cards_to_add
                .map(|cta| {
                    let mut cards_vec = cards_without_jokers.clone();
                    cards_vec.append(&mut cta.clone());

                    let h = Hand {
                        cards: cards_vec.try_into().unwrap(),
                        bid: self.bid,
                    };
                    h.calc_hand_type_without_joker()
                })
                .max()
                .unwrap()
        } else {
            self.calc_hand_type_without_joker()
        }
    }

    /// Calculate hand type assuming that there are no jokers (or jokers already have been substituted).
    fn calc_hand_type_without_joker(&self) -> HandType {
        // Card how often types of cards appear in this hand
        let mut card_counts: BTreeMap<Card, u8> = BTreeMap::new();

        for c in self.cards {
            *card_counts.entry(c).or_default() += 1;
        }

        let count_values: Vec<u8> = card_counts.values().cloned().collect();

        if count_values.contains(&5) {
            HandType::FiveOfAKind
        } else if count_values.contains(&4) {
            HandType::FourOfAKind
        } else if count_values.contains(&3) && count_values.contains(&2) {
            HandType::FullHouse
        } else if count_values.contains(&3) {
            HandType::ThreeOfAKind
        } else if count_values.iter().filter(|n| **n == 2).count() == 2 {
            HandType::TwoPair
        } else if count_values.contains(&2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn calc_total_winnings(mut hands: Vec<Hand>) -> usize {
    // Sort hands by ascending strength
    hands.sort_unstable();

    // For debugging
    /* for h in &hands {
        println!(
            "{:?}: {:?} (bid {})",
            h.cards,
            h.calc_hand_type_with_replaced_jokers(),
            h.bid
        )
    } */

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum()
}

fn main() -> Result<()> {
    let input_without_jokers = read_input_file("../inputs/day7_input.txt", false)?;
    let input_with_jokers = read_input_file("../inputs/day7_input.txt", true)?;

    println!(
        "Total winnings without jokers: {}",
        calc_total_winnings(input_without_jokers)
    );
    println!(
        "Total winnings with jokers: {}",
        calc_total_winnings(input_with_jokers)
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P, parse_j_as_joker: bool) -> Result<Vec<Hand>> {
    let input = read_to_string(input_path)?;
    let res: Vec<Result<_>> = input
        .lines()
        .map(|l| parse_input_line(l, parse_j_as_joker))
        .collect();
    res.into_iter().collect()
}

fn parse_input_line(line: &str, parse_j_as_joker: bool) -> Result<Hand> {
    let mut elements = line.split(' ');

    let cards_str = elements.next().context("Could not extract hand string!")?;
    ensure!(cards_str.len() == 5, "Hand does not consist of five cards!");
    let cards_vec: Vec<_> = cards_str
        .chars()
        .filter_map(|c| {
            if parse_j_as_joker && c == 'J' {
                Some(Card::Joker)
            } else {
                c.try_into().ok()
            }
        })
        .collect();
    ensure!(cards_vec.len() == 5, "Card could not be parsed!");

    let bid_str = elements.next().context("Could not extract bid!")?;

    Ok(Hand {
        cards: cards_vec.try_into().unwrap(),
        bid: bid_str
            .parse()
            .context("Could not parse bid as a number!")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day7_example.txt", false).unwrap();
        assert_eq!(calc_total_winnings(input), 6440);
    }

    #[test]
    fn example_second_star() {
        let input = read_input_file("../inputs/day7_example.txt", true).unwrap();
        assert_eq!(calc_total_winnings(input), 5905);
    }
}
