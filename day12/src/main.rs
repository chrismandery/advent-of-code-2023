use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

fn calc_possible_arrangements(input: &[Condition], block_lengths: &[usize]) -> usize {
    // If we are done, return 1
    if input.is_empty() && block_lengths.is_empty() {
        return 1;
    }

    // Determine count by dynamic programing
    let mut count = 0;

    // Do we expect at least one more block and can we put this at the beginning of our condition list?
    if let Some(next_block_length) = block_lengths.first() {
        if input.len() >= *next_block_length
            && input[..*next_block_length]
                .iter()
                .all(|c| *c == Condition::Damaged || *c == Condition::Unknown)
            && *input.get(*next_block_length).unwrap_or(&Condition::Unknown) != Condition::Damaged
        {
            let mut new_input = input[*next_block_length..].to_vec();
            if let Some(first) = new_input.first_mut() {
                if *first == Condition::Unknown {
                    *first = Condition::Operational;
                }
            }

            let new_block_lengths = block_lengths[1..].to_vec();

            count += calc_possible_arrangements(&new_input, &new_block_lengths);
        }
    }

    // If we are not a damaged field, we can always assume the current first field is not part of a block
    if let Some(first) = input.first() {
        if *first != Condition::Damaged {
            let new_input = input[1..].to_vec();
            count += calc_possible_arrangements(&new_input, block_lengths);
        }
    }

    count
}

fn calc_possible_arrangements_wrapper(input: &(Vec<Condition>, Vec<usize>)) -> usize {
    calc_possible_arrangements(&input.0, &input.1)
}

fn main() {
    let input = read_input_file("../inputs/day12_input.txt");

    println!(
        "Sum of all possible arrangement counts: {}",
        input
            .iter()
            .map(calc_possible_arrangements_wrapper)
            .sum::<usize>()
    );
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Vec<(Vec<Condition>, Vec<usize>)> {
    let input = read_to_string(input_path).expect("Could not open file!");

    input
        .lines()
        .map(|l| {
            let (con_str, len_str) = l.split_once(' ').expect("Could not split line!");
            let condition_list = con_str
                .chars()
                .map(|c| match c {
                    '.' => Condition::Operational,
                    '#' => Condition::Damaged,
                    '?' => Condition::Unknown,
                    _ => panic!("Unknown character!"),
                })
                .collect();
            let damaged_block_lengths = len_str.split(',').map(|s| s.parse().unwrap()).collect();

            (condition_list, damaged_block_lengths)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day12_example.txt");
        let mut it = input.iter();
        assert_eq!(calc_possible_arrangements_wrapper(it.next().unwrap()), 1);
        assert_eq!(calc_possible_arrangements_wrapper(it.next().unwrap()), 4);
        assert_eq!(calc_possible_arrangements_wrapper(it.next().unwrap()), 1);
        assert_eq!(calc_possible_arrangements_wrapper(it.next().unwrap()), 1);
        assert_eq!(calc_possible_arrangements_wrapper(it.next().unwrap()), 4);
        assert_eq!(calc_possible_arrangements_wrapper(it.next().unwrap()), 10);
    }
}
