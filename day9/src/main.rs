use std::fs::read_to_string;
use std::path::Path;

fn get_next_value_for_history(n: &[i64]) -> i64 {
    let diffs: Vec<i64> = n.windows(2).map(|w| w[1] - w[0]).collect();

    let increment = if diffs.iter().all(|d| *d == 0) {
        // If all diffs are zero, the next value is the last element
        0
    } else {
        // Calculate increment based on next value for diff history
        get_next_value_for_history(&diffs)
    };

    n.last().unwrap() + increment
}

fn main() {
    let input = read_input_file("../inputs/day9_input.txt");

    println!(
        "Sum of all extrapolated values: {}",
        input
            .iter()
            .map(|n| get_next_value_for_history(n))
            .sum::<i64>()
    );
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Vec<Vec<i64>> {
    let input = read_to_string(input_path).expect("Could not open file!");
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i64>().expect("Could not parse number?!"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day9_example.txt");
        assert_eq!(
            input
                .iter()
                .map(|n| {
                    let x = get_next_value_for_history(n);
                    dbg!(&x);
                    x
                })
                .sum::<i64>(),
            114
        );
    }
}
