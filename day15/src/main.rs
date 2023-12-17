use anyhow::{bail, Context, Result};
use std::fs::read_to_string;
use std::path::Path;

fn hash(s: &str) -> u8 {
    let mut cur = 0;

    for c in s.chars() {
        cur += c as u32;
        cur *= 17;
        cur %= 256;
    }

    cur as u8
}

fn main() -> Result<()> {
    let input = process_input_file("../inputs/day15_input.txt")?;
    let input_ref: Vec<&str> = input.iter().map(|s| &s as &str).collect();

    println!(
        "Sum of hashes for first star: {}",
        input.iter().map(|s| hash(s) as u32).sum::<u32>()
    );

    println!(
        "Focusing power for second star: {}",
        run_hash_boxes(&input_ref)?
    );

    Ok(())
}

fn process_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<String>> {
    let input = read_to_string(input_path)?;
    let line = input.lines().next().context("Could not read line!")?;
    let res = line.split(',').map(|s| s.to_string()).collect();
    Ok(res)
}

/// Runs the hash boxes algorithm necessary for the second step and returns the "focusing power".
fn run_hash_boxes(input: &[&str]) -> Result<usize> {
    let mut boxes: [Vec<(&str, u8)>; 256] = vec![Vec::new(); 256].try_into().unwrap();

    for s in input {
        if let Some((label, fl)) = s.split_once('=') {
            let cur_box = &mut boxes[hash(label) as usize];
            let fl = fl.parse().context("Could not parse number!")?;

            if let Some(pos) = cur_box.iter().position(|(l, _)| *l == label) {
                cur_box[pos].1 = fl;
            } else {
                cur_box.push((&label, fl));
            }
        } else if s.ends_with('-') {
            let label = &s[0..(s.len() - 1)];
            let cur_box = &mut boxes[hash(label) as usize];
            if let Some(pos) = cur_box.iter().position(|(l, _)| *l == label) {
                cur_box.remove(pos);
            }
        } else {
            bail!("Could not parse input step!");
        }
    }

    let mut focusing_power = 0;

    for (i, cur_box) in boxes.iter().enumerate() {
        for (j, (_, fl)) in cur_box.iter().enumerate() {
            focusing_power += (i + 1) * (j + 1) * *fl as usize;
        }
    }

    Ok(focusing_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = process_input_file("../inputs/day15_example.txt").unwrap();
        let mut it = input.iter();

        assert_eq!(hash(it.next().unwrap()), 30);
        assert_eq!(hash(it.next().unwrap()), 253);
        assert_eq!(hash(it.next().unwrap()), 97);
        assert_eq!(hash(it.next().unwrap()), 47);
        assert_eq!(hash(it.next().unwrap()), 14);
        assert_eq!(hash(it.next().unwrap()), 180);
        assert_eq!(hash(it.next().unwrap()), 9);
        assert_eq!(hash(it.next().unwrap()), 197);
        assert_eq!(hash(it.next().unwrap()), 48);
        assert_eq!(hash(it.next().unwrap()), 214);
        assert_eq!(hash(it.next().unwrap()), 231);
    }

    #[test]
    fn example_second_star() {
        let input = process_input_file("../inputs/day15_example.txt").unwrap();
        let input_ref: Vec<&str> = input.iter().map(|s| &s as &str).collect();
        assert_eq!(run_hash_boxes(&input_ref).unwrap(), 145);
    }
}
