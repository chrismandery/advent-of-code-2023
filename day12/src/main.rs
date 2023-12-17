use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

type Cache = HashMap<(Vec<Condition>, Vec<usize>), usize>;

fn calc_possible_arrangements(
    cache: &mut Cache,
    input: &[Condition],
    block_lengths: &[usize],
) -> usize {
    // If we are done, return 1
    if input.is_empty() && block_lengths.is_empty() {
        return 1;
    }

    // Check cache (hacked with slow vector creation)
    if let Some(res) = cache.get(&(input.to_vec(), block_lengths.to_vec())) {
        return *res;
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

            count += calc_possible_arrangements(cache, &new_input, &new_block_lengths);
        }
    }

    // If we are not a damaged field, we can always assume the current first field is not part of a block
    if let Some(first) = input.first() {
        if *first != Condition::Damaged {
            let new_input = input[1..].to_vec();
            count += calc_possible_arrangements(cache, &new_input, block_lengths);
        }
    }

    cache.insert((input.to_vec(), block_lengths.to_vec()), count);

    count
}

fn calc_possible_arrangements_wrapper(
    input: &(Vec<Condition>, Vec<usize>),
    unfold_five_times: bool,
) -> usize {
    if unfold_five_times {
        let mut input_new = input.0.clone();
        let mut block_lengths_new = input.1.clone();

        for _ in 0..4 {
            input_new.push(Condition::Unknown);
            input_new.append(&mut input.0.clone());
            block_lengths_new.append(&mut input.1.clone());
        }

        calc_possible_arrangements(&mut Cache::new(), &input_new, &block_lengths_new)
    } else {
        calc_possible_arrangements(&mut Cache::new(), &input.0, &input.1)
    }
}

fn main() {
    let input = read_input_file("../inputs/day12_input.txt");

    println!(
        "Sum of all possible arrangement counts (first star): {}",
        input
            .iter()
            .map(|x| calc_possible_arrangements_wrapper(x, false))
            .sum::<usize>()
    );

    println!(
        "Sum of all possible arrangement counts (second star): {}",
        input
            .par_iter()
            .progress_count(input.len() as u64)
            .map(|x| calc_possible_arrangements_wrapper(x, true))
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
        assert_eq!(
            calc_possible_arrangements_wrapper(it.next().unwrap(), false),
            1
        );
        assert_eq!(
            calc_possible_arrangements_wrapper(it.next().unwrap(), false),
            4
        );
        assert_eq!(
            calc_possible_arrangements_wrapper(it.next().unwrap(), false),
            1
        );
        assert_eq!(
            calc_possible_arrangements_wrapper(it.next().unwrap(), false),
            1
        );
        assert_eq!(
            calc_possible_arrangements_wrapper(it.next().unwrap(), false),
            4
        );
        assert_eq!(
            calc_possible_arrangements_wrapper(it.next().unwrap(), false),
            10
        );
    }

    #[test]
    fn example_second_star() {
        let input = read_input_file("../inputs/day12_example.txt");
        let mut it = input.iter();
        assert_eq!(
            calc_possible_arrangements_wrapper(it.next().unwrap(), true),
            1
        );
        assert_eq!(
            calc_possible_arrangements_wrapper(it.next().unwrap(), true),
            16384
        );
        assert_eq!(
            calc_possible_arrangements_wrapper(it.next().unwrap(), true),
            1
        );
        assert_eq!(
            calc_possible_arrangements_wrapper(it.next().unwrap(), true),
            16
        );
        assert_eq!(
            calc_possible_arrangements_wrapper(it.next().unwrap(), true),
            2500
        );
        assert_eq!(
            calc_possible_arrangements_wrapper(it.next().unwrap(), true),
            506250
        );
    }
}
