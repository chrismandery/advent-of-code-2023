use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

struct Input {
    condition_list: Vec<Condition>,
    damaged_block_lengths: Vec<usize>,
}

fn calc_possible_arrangements(input: &Input) -> usize {
    // Determine count by dynamic programing
    let mut count = 0;

    // If we are done, return 1
    if input.condition_list.is_empty() && input.damaged_block_lengths.is_empty() {
        return 1;
    }

    // Do we expect at least one more block and can we put this at the beginning of our condition list?
    if let Some(next_block_length) = input.damaged_block_lengths.first() {
        if input.condition_list.len() >= *next_block_length
            && input.condition_list[..*next_block_length]
                .iter()
                .all(|c| *c == Condition::Damaged || *c == Condition::Unknown)
        {
            let remaining_blocks = input.damaged_block_lengths[1..].to_vec();
            let mut remaining_condition_list = input.condition_list[*next_block_length..].to_vec();
            if let Some(first) = remaining_condition_list.first_mut() {
                if *first == Condition::Unknown {
                    *first = Condition::Operational;
                }
            }

            count += calc_possible_arrangements(&Input {
                condition_list: remaining_condition_list,
                damaged_block_lengths: remaining_blocks,
            });
        }
    }

    // If we are not a damaged field, we can always assume the current first field is not part of a block
    if let Some(first) = input.condition_list.first() {
        if *first != Condition::Damaged {
            let remaining_condition_list = input.condition_list[1..].to_vec();

            count += calc_possible_arrangements(&Input {
                condition_list: remaining_condition_list,
                damaged_block_lengths: input.damaged_block_lengths.clone(),
            });
        }
    }

    count
}

fn main() {
    let input = read_input_file("../inputs/day12_input.txt");

    println!(
        "Sum of all possible arrangement counts: {}",
        input.iter().map(calc_possible_arrangements).sum::<usize>()
    );
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Vec<Input> {
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

            Input {
                condition_list,
                damaged_block_lengths,
            }
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
        assert_eq!(calc_possible_arrangements(it.next().unwrap()), 1);
        assert_eq!(calc_possible_arrangements(it.next().unwrap()), 4);
        assert_eq!(calc_possible_arrangements(it.next().unwrap()), 1);
        assert_eq!(calc_possible_arrangements(it.next().unwrap()), 1);
        assert_eq!(calc_possible_arrangements(it.next().unwrap()), 4);
        assert_eq!(calc_possible_arrangements(it.next().unwrap()), 10);
    }
}
