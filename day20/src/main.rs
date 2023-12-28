use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
enum NodeState {
    Broadcaster,
    FlipFlop { cur_state: bool },
    Conjunction { input_states: HashMap<String, bool> },
}

#[derive(Debug)]
struct Node {
    destinations: Vec<String>,
    state: NodeState,
}

impl Node {
    fn process_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        let output_value = match self.state {
            NodeState::Broadcaster => pulse.value,
            NodeState::FlipFlop { mut cur_state } => {
                if !pulse.value {
                    cur_state = !cur_state;
                }
                cur_state
            }
            NodeState::Conjunction {
                ref mut input_states,
            } => {
                input_states
                    .entry(pulse.source.clone())
                    .and_modify(|x| {
                        if !pulse.value {
                            *x = !*x
                        }
                    })
                    .or_insert_with(|| !pulse.value);

                !input_states.values().all(|x| *x)
            }
        };

        self.destinations
            .iter()
            .map(|d| Pulse {
                source: pulse.destination.to_string(),
                destination: d.to_string(),
                value: output_value,
            })
            .collect()
    }
}

struct Pulse {
    source: String,
    destination: String,
    value: bool,
}

type Nodes = HashMap<String, Node>;

/// Pushes the button and processes all pulses, returning the number of high and low pulses.
fn push_button(nodes: &mut Nodes) -> (usize, usize) {
    let mut pulse_queue = VecDeque::new();
    pulse_queue.push_back(Pulse {
        source: "".to_string(),
        destination: "broadcaster".to_string(),
        value: false,
    });

    let pulse_count = (0, 0);

    while let Some(pulse) = pulse_queue.pop_front() {
        // Debug print
        if pulse.value {
            println!("{} -high-> {}", pulse.source, pulse.destination);
        } else {
            println!("{} -low-> {}", pulse.source, pulse.destination);
        }

        if pulse.destination == "output" {
            continue; // Ignore pulses going to output
        }

        let n = nodes
            .get_mut(&pulse.destination)
            .unwrap_or_else(|| panic!("Node {} not found!", pulse.destination));
        let new_pulses = n.process_pulse(&pulse);
        pulse_queue.append(&mut VecDeque::from(new_pulses));

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    pulse_count
}

/// Pushes the button n times, returning the number of high and low pulses.
fn push_button_n_times(nodes: &mut Nodes, n: usize) -> (usize, usize) {
    let mut counts = (0, 0);
    for _ in 0..n {
        let upd = push_button(nodes);
        counts.0 += upd.0;
        counts.1 += upd.1;
    }
    counts
}

fn main() -> Result<()> {
    let mut nodes = read_input_file("../inputs/day20_input.txt")?;

    let counts = push_button_n_times(&mut nodes, 1000);
    println!(
        "Product of high and low pulse counts (first star): {}",
        counts.0 * counts.1
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Nodes> {
    let re = Regex::new(r"^([a-z%&]+) -> ([a-z ,]+)$").unwrap();

    let input = read_to_string(input_path).expect("Could not open file!");
    let res = input
        .lines()
        .filter_map(|l| {
            if let Some(cap) = re.captures(l) {
                let name_raw = cap.get(1).unwrap().as_str();
                let dest_raw = cap.get(2).unwrap().as_str();

                let (name, state) = if let Some(stripped) = name_raw.strip_prefix('%') {
                    (
                        stripped.to_string(),
                        NodeState::FlipFlop { cur_state: false },
                    )
                } else if let Some(stripped) = name_raw.strip_prefix('&') {
                    (
                        stripped.to_string(),
                        NodeState::Conjunction {
                            input_states: HashMap::new(),
                        },
                    )
                } else {
                    (name_raw.to_string(), NodeState::Broadcaster)
                };

                let destinations = dest_raw.split(", ").map(|s| s.to_string()).collect();

                Some((
                    name,
                    Node {
                        destinations,
                        state,
                    },
                ))
            } else {
                println!("Could not parse input line: {}", l);
                None
            }
        })
        .collect();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut nodes = read_input_file("../inputs/day20_example1.txt").unwrap();
        assert_eq!(push_button_n_times(&mut nodes, 1000), (4000, 8000));
    }

    #[test]
    fn test_example2() {
        let mut nodes = read_input_file("../inputs/day20_example2.txt").unwrap();
        assert_eq!(push_button_n_times(&mut nodes, 1000), (2750, 4250));
    }
}
