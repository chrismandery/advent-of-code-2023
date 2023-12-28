use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, Debug)]
enum NodeState {
    Broadcaster,
    FlipFlop { cur_state: bool },
    Conjunction { input_states: HashMap<String, bool> },
}

#[derive(Clone, Debug)]
struct Node {
    destinations: Vec<String>,
    state: NodeState,
}

impl Node {
    fn process_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        let output_value = match self.state {
            NodeState::Broadcaster => Some(pulse.value),
            NodeState::FlipFlop { ref mut cur_state } => {
                if pulse.value {
                    None
                } else {
                    *cur_state = !*cur_state;
                    Some(*cur_state)
                }
            }
            NodeState::Conjunction {
                ref mut input_states,
            } => {
                let v = input_states
                    .get_mut(&pulse.source)
                    .expect("Source not found for conjunction!");
                *v = pulse.value;

                Some(!input_states.values().all(|x| *x))
            }
        };

        if let Some(val) = output_value {
            self.destinations
                .iter()
                .map(|d| Pulse {
                    source: pulse.destination.to_string(),
                    destination: d.to_string(),
                    value: val,
                })
                .collect()
        } else {
            vec![]
        }
    }
}

struct Pulse {
    source: String,
    destination: String,
    value: bool,
}

type Nodes = HashMap<String, Node>;

/// Pushes the button and processes all pulses, returning the number of high and low pulses. Optionally, abort if the node given in the
/// second parameter sends a high value (if this happens, this is indicated in the third return value).
fn push_button(
    nodes: &mut Nodes,
    abort_if_node_sends_high_pulse: Option<&str>,
) -> (usize, usize, bool) {
    let mut pulse_queue = VecDeque::new();
    pulse_queue.push_back(Pulse {
        source: "".to_string(),
        destination: "broadcaster".to_string(),
        value: false,
    });

    let (mut pulse_count_high, mut pulse_count_low) = (0, 0);

    while let Some(pulse) = pulse_queue.pop_front() {
        // Debug print
        /* if pulse.value {
            println!("{} -high-> {}", pulse.source, pulse.destination);
        } else {
            println!("{} -low-> {}", pulse.source, pulse.destination);
        } */

        if pulse.value {
            pulse_count_high += 1;
        } else {
            pulse_count_low += 1;
        }

        if pulse.destination == "output" || pulse.destination == "rx" {
            continue; // Ignore pulses going to output
        }

        if let Some(abort_node) = abort_if_node_sends_high_pulse {
            if pulse.source == abort_node && pulse.value {
                return (pulse_count_high, pulse_count_low, true);
            }
        }

        let n = nodes
            .get_mut(&pulse.destination)
            .unwrap_or_else(|| panic!("Node {} not found!", pulse.destination));
        let new_pulses = n.process_pulse(&pulse);
        pulse_queue.append(&mut VecDeque::from(new_pulses));
    }

    (pulse_count_high, pulse_count_low, false)
}

/// Pushes the button n times, returning the number of high and low pulses.
fn push_button_n_times(nodes: &mut Nodes, n: usize) -> (usize, usize) {
    let mut counts = (0, 0);
    for _ in 0..n {
        let (upd_high, upd_low, _) = push_button(nodes, None);
        counts.0 += upd_high;
        counts.1 += upd_low;
    }
    counts
}

/// Counts the buttom presses that are necessary until the given node sends a high pulse.
fn push_button_until_node_sends_high_pulse(nodes: &mut Nodes, node_name: &str) -> usize {
    let mut n = 1;
    while !push_button(nodes, Some(node_name)).2 {
        n += 1;
    }
    n
}

fn main() -> Result<()> {
    // First star
    let mut nodes = read_input_file("../inputs/day20_input.txt")?;
    let counts = push_button_n_times(&mut nodes, 1000);

    println!(
        "Product of high and low pulse counts (first star): {}",
        counts.0 * counts.1
    );

    // Second star
    // Use a list of nodes (hardcoded here) that must send a high pulse for a low pulse being sent to rx
    // The solution is then the product of these button push counts (technically it should be LCM, but here it looks like LCM == product)
    let rx_input_nodes = ["sr", "sn", "rf", "vq"];
    let mut button_push_counts = vec![];

    for node in rx_input_nodes {
        let mut nodes = read_input_file("../inputs/day20_input.txt")?;
        button_push_counts.push(push_button_until_node_sends_high_pulse(&mut nodes, node));
    }

    println!(
        "Button presses required for rx low pulse: {}",
        button_push_counts.iter().product::<usize>()
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Nodes> {
    let re = Regex::new(r"^([a-z%&]+) -> ([a-z ,]+)$").unwrap();

    let input = read_to_string(input_path).expect("Could not open file!");
    let mut res: Nodes = input
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

    // Initialize conjunction nodes
    let res_clone = res.clone();
    for (name, node) in res.iter_mut() {
        if let NodeState::Conjunction {
            ref mut input_states,
        } = node.state
        {
            for source_name in res_clone
                .iter()
                .filter(|(_, n)| n.destinations.contains(name))
                .map(|(source_name, _)| source_name)
            {
                input_states.insert(source_name.to_string(), false);
            }
        }
    }

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
