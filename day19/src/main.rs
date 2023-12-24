use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct Rule {
    category: u8,
    has_to_be_larger: bool,
    threshold: u64,
    target: String,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    default_target: String,
}

type WorkflowList = HashMap<String, Workflow>;
type Part = [u64; 4];
type PartRange = [(u64, u64); 4];

fn calc_combinations(pr: &PartRange) -> u64 {
    (pr[0].1 - pr[0].0 + 1)
        * (pr[1].1 - pr[1].0 + 1)
        * (pr[2].1 - pr[2].0 + 1)
        * (pr[3].1 - pr[3].0 + 1)
}

fn check_part_accepted(wl: &WorkflowList, part: &Part) -> bool {
    check_part_accepted_recursive(wl, part, "in", 0)
}

fn check_part_accepted_recursive(
    wl: &WorkflowList,
    part: &Part,
    workflow_name: &str,
    rule_num: usize,
) -> bool {
    let workflow = wl
        .get(workflow_name)
        .unwrap_or_else(|| panic!("Workflow {} not found!", workflow_name));

    if let Some(rule) = workflow.rules.get(rule_num) {
        let value = part[rule.category as usize];

        let is_fulfilled = if rule.has_to_be_larger {
            value > rule.threshold
        } else {
            value < rule.threshold
        };

        if is_fulfilled {
            if rule.target == "A" {
                true
            } else if rule.target == "R" {
                false
            } else {
                check_part_accepted_recursive(wl, part, &rule.target, 0)
            }
        } else {
            check_part_accepted_recursive(wl, part, workflow_name, rule_num + 1)
        }
    } else if workflow.default_target == "A" {
        true
    } else if workflow.default_target == "R" {
        false
    } else {
        check_part_accepted_recursive(wl, part, &workflow.default_target, 0)
    }
}

fn count_accepted_parts(wl: &WorkflowList, pr: &PartRange) -> u64 {
    count_accepted_parts_recursive(wl, pr, "in", 0)
}

fn count_accepted_parts_recursive(
    wl: &WorkflowList,
    pr: &PartRange,
    workflow_name: &str,
    rule_num: usize,
) -> u64 {
    if workflow_name == "A" {
        return calc_combinations(pr);
    } else if workflow_name == "R" {
        return 0;
    };

    let workflow = wl
        .get(workflow_name)
        .unwrap_or_else(|| panic!("Workflow {} not found!", workflow_name));

    if let Some(rule) = workflow.rules.get(rule_num) {
        let range = pr[rule.category as usize];

        #[allow(clippy::collapsible_else_if)]
        if rule.has_to_be_larger {
            if range.0 > rule.threshold {
                // The whole range is above the threshold -> the condition is fulfilled in all cases
                count_accepted_parts_recursive(wl, pr, &rule.target, 0)
            } else if range.1 <= rule.threshold {
                // The whole range is below/equal the threshold -> the condition is fulfilled in none of the cases
                count_accepted_parts_recursive(wl, pr, workflow_name, rule_num + 1)
            } else {
                // Split interval into two cases and handle them separately
                let mut pr_true = *pr;
                pr_true[rule.category as usize].0 = rule.threshold + 1;

                let mut pr_false = *pr;
                pr_false[rule.category as usize].1 = rule.threshold;

                count_accepted_parts_recursive(wl, &pr_true, &rule.target, 0)
                    + count_accepted_parts_recursive(wl, &pr_false, workflow_name, rule_num + 1)
            }
        } else {
            if range.1 < rule.threshold {
                // The whole range is below the threshold -> the condition is fulfilled in all cases
                count_accepted_parts_recursive(wl, pr, &rule.target, 0)
            } else if range.0 >= rule.threshold {
                // The whole range is above/equal the threshold -> the condition is fulfilled in none of the cases
                count_accepted_parts_recursive(wl, pr, workflow_name, rule_num + 1)
            } else {
                // Split interval into two cases and handle them separately
                let mut pr_true = *pr;
                pr_true[rule.category as usize].1 = rule.threshold - 1;

                let mut pr_false = *pr;
                pr_false[rule.category as usize].0 = rule.threshold;

                count_accepted_parts_recursive(wl, &pr_true, &rule.target, 0)
                    + count_accepted_parts_recursive(wl, &pr_false, workflow_name, rule_num + 1)
            }
        }
    } else {
        count_accepted_parts_recursive(wl, pr, &workflow.default_target, 0)
    }
}

fn get_accepted_parts_category_sum(wl: &WorkflowList, parts: &[Part]) -> u64 {
    parts
        .iter()
        .filter(|part| check_part_accepted(wl, part))
        .map(|part| part.iter().sum::<u64>())
        .sum()
}

fn main() -> Result<()> {
    let (workflows, parts) = read_input_file("../inputs/day19_input.txt")?;

    println!(
        "Sum of categories for all accepted parts (first star): {}",
        get_accepted_parts_category_sum(&workflows, &parts)
    );

    println!(
        "Total number of accepted parts (second star): {}",
        count_accepted_parts(&workflows, &[(1, 4000), (1, 4000), (1, 4000), (1, 4000)])
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<(WorkflowList, Vec<Part>)> {
    let input = read_to_string(input_path)?;
    let lines: Vec<_> = input.lines().collect();
    let mut sections = lines.split(|l| l.is_empty());

    let workflow_section = sections.next().ok_or(anyhow!("Early EOF?"))?;
    let mut workflows = WorkflowList::new();
    let workflow_re = Regex::new(r"^([[:alpha:]]+)\{(.+),([[:alpha:]]+)\}$").unwrap();
    let parts_re = Regex::new(r"^([xmas])([<>])(\d+):([[:alpha:]]+)$").unwrap();

    for line in workflow_section {
        let cap = workflow_re
            .captures(line)
            .ok_or(anyhow!("Could not match workflow line!"))?;

        let name = cap.get(1).unwrap().as_str().to_owned();
        let default_target = cap.get(3).unwrap().as_str().to_owned();

        let mut rules = vec![];
        for rule_str in cap.get(2).unwrap().as_str().split(',') {
            let cap = parts_re
                .captures(rule_str)
                .ok_or(anyhow!("Could not match rule!"))?;

            let category = match cap.get(1).unwrap().as_str() {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                _ => 3,
            };
            let has_to_be_larger = cap.get(2).unwrap().as_str() == ">";
            let threshold = cap.get(3).unwrap().as_str().parse().unwrap();
            let target = cap.get(4).unwrap().as_str().to_owned();

            let rule = Rule {
                category,
                has_to_be_larger,
                threshold,
                target,
            };

            rules.push(rule);
        }

        let w = Workflow {
            rules,
            default_target,
        };

        workflows.insert(name, w);
    }

    let parts_section = sections.next().ok_or(anyhow!("Early EOF?"))?;
    let mut parts = vec![];
    let part_re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();

    for line in parts_section {
        let cap = part_re
            .captures(line)
            .ok_or(anyhow!("Could not match part line!"))?;
        let part = [
            cap.get(1).unwrap().as_str().parse().unwrap(),
            cap.get(2).unwrap().as_str().parse().unwrap(),
            cap.get(3).unwrap().as_str().parse().unwrap(),
            cap.get(4).unwrap().as_str().parse().unwrap(),
        ];
        parts.push(part);
    }

    Ok((workflows, parts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let (workflows, parts) = read_input_file("../inputs/day19_example.txt").unwrap();
        assert!(check_part_accepted(&workflows, &parts[0]));
        assert!(!check_part_accepted(&workflows, &parts[1]));
        assert!(check_part_accepted(&workflows, &parts[2]));
        assert!(!check_part_accepted(&workflows, &parts[3]));
        assert!(check_part_accepted(&workflows, &parts[4]));
        assert_eq!(get_accepted_parts_category_sum(&workflows, &parts), 19114);
    }

    #[test]
    fn example_second_star() {
        let (workflows, _) = read_input_file("../inputs/day19_example.txt").unwrap();
        assert_eq!(
            count_accepted_parts(&workflows, &[(1, 4000), (1, 4000), (1, 4000), (1, 4000)]),
            167409079868000
        );
    }
}
