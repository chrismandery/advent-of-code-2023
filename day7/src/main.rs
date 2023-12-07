use anyhow::{ensure, Context, Result};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(c: char) -> Result<Card, Self::Error> {
        match c {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::N9),
            '8' => Ok(Card::N8),
            '7' => Ok(Card::N7),
            '6' => Ok(Card::N6),
            '5' => Ok(Card::N5),
            '4' => Ok(Card::N4),
            '3' => Ok(Card::N3),
            '2' => Ok(Card::N2),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ht_self = self.calc_hand_type();
        let ht_other = other.calc_hand_type();

        let cmp = ht_self.cmp(&ht_other);
        match cmp {
            Ordering::Equal => {
                // Run tie-breaker
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    if c1 < c2 {
                        return Ordering::Less;
                    } else if c1 > c2 {
                        return Ordering::Greater;
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
    fn calc_hand_type(&self) -> HandType {
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

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn calc_total_winnings(mut hands: Vec<Hand>) -> usize {
    // Sort hands by ascending strength
    hands.sort_unstable();
    hands.iter().enumerate().map(|(i, hand)| hand.bid * i).sum()
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day7_input.txt")?;

    println!("Total winnings: {}", calc_total_winnings(input));

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<Hand>> {
    let input = read_to_string(input_path)?;
    let res: Vec<Result<_>> = input.lines().map(parse_input_line).collect();
    res.into_iter().collect()
}

fn parse_input_line(line: &str) -> Result<Hand> {
    let mut elements = line.split(' ');

    let cards_str = elements.next().context("Could not extract hand string!")?;
    ensure!(cards_str.len() == 5, "Hand does not consist of five cards!");
    let cards_vec: Vec<_> = cards_str
        .chars()
        .filter_map(|c| c.try_into().ok())
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
        let input = read_input_file("../inputs/day7_example.txt").unwrap();
        assert_eq!(calc_total_winnings(input), 6440);
    }
}
