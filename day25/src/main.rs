use anyhow::{anyhow, Context, Result};
use rand::prelude::*;
use rayon::prelude::*;
use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::path::Path;

type GraphEdge = (BTreeSet<String>, BTreeSet<String>);

/// The graph is undirected and we store all egdes A-B in the form such that A < B. Note that the graph is not a set but just a list since
/// the same edge can be repeated multiple times.
type Graph = Vec<GraphEdge>;

const MAX_TRIES: usize = 1000;

/// Returns the product of the two group sizes after a min-cut with the given number of cuts is found.
#[allow(dead_code)]
fn find_mincut_product(g: &Graph, n_cuts: usize) -> usize {
    loop {
        let mut g_copy = g.clone();
        karger_algo_mincut(&mut g_copy);

        println!(
            "Karger's algorithm found solution with {} cuts.",
            g_copy.len()
        );

        if g_copy.len() == n_cuts {
            println!("Found solution!");
            let edges = g_copy.first().unwrap();
            return edges.0.len() * edges.1.len();
        }
    }
}

/// Parallelized version (outputs the result directly to simplify the code with Rayon).
fn find_mincut_product_parallelized(g: &Graph, n_cuts: usize) {
    (0..MAX_TRIES).into_par_iter().for_each(|_| {
        let mut g_copy = g.clone();
        karger_algo_mincut(&mut g_copy);

        println!(
            "Karger's algorithm found solution with {} cuts.",
            g_copy.len()
        );

        if g_copy.len() == n_cuts {
            let edges = g_copy.first().unwrap();

            println!("Found solution!");

            println!(
                "Product of group sizes after optimal min-cut: {}",
                edges.0.len() * edges.1.len()
            );
            panic!(); // Ultra-ugly hack to abort...
        }
    })
}

/// Karger's algorithm (https://en.wikipedia.org/wiki/Karger%27s_algorithm), which is a probabilistic algorithm that has some change to
/// find a min-cut (for this problem it seems to work reasonably well). The given graph is modified in-place and at the end represents the
/// min-cut with two nodes and the number of edges between those nodes representing the edges that are part of the min-cut.
fn karger_algo_mincut(g: &mut Graph) {
    let mut rng = rand::thread_rng();

    loop {
        // Debug print all edges
        /* println!("\nEdges of the current graph:");
        for (a, b) in g.iter() {
            println!("{} - {}", a.iter().join("/"), b.iter().join("/"));
        } */

        // Check that condition "we store edges A-B where A<B always holds"
        assert!(g.iter().all(|(a, b)| a < b));

        // Abort when there are only two nodes left
        let source_nodes: BTreeSet<BTreeSet<String>> = g.iter().map(|(a, _)| a).cloned().collect();
        let dest_nodes: BTreeSet<BTreeSet<String>> = g.iter().map(|(_, b)| b).cloned().collect();
        let all_nodes: BTreeSet<BTreeSet<String>> =
            source_nodes.union(&dest_nodes).cloned().collect();

        if all_nodes.len() == 2 {
            /* println!("Aborting, there are only two nodes left:");
            println!("{:?}", all_nodes); */
            return;
        }

        // Sample random edge from the graph
        let edge_num = rng.gen_range(0..g.len());
        let collapse_edge = g.get(edge_num).cloned().unwrap();

        // Remove this edge from the graph
        g.retain(|edge| *edge != collapse_edge);

        /* println!(
            "\nCollapsing this edge: {} - {}",
            collapse_edge.0.iter().join("/"),
            collapse_edge.1.iter().join("/")
        ); */

        // Build new merged node that contains all nodes from the merged edge
        let merged_nodes: BTreeSet<String> =
            collapse_edge.0.union(&collapse_edge.1).cloned().collect();

        // Filter out all edges that contain one of the two to-be-merged nodes and create new edges for the merged new
        let mut new_edges = Graph::new();

        g.retain(|(a, b)| {
            let new_edge = if *a == collapse_edge.0 || *a == collapse_edge.1 {
                if merged_nodes < *b {
                    Some((merged_nodes.clone(), b.clone()))
                } else {
                    Some((b.clone(), merged_nodes.clone()))
                }
            } else if *b == collapse_edge.0 || *b == collapse_edge.1 {
                if merged_nodes < *a {
                    Some((merged_nodes.clone(), a.clone()))
                } else {
                    Some((a.clone(), merged_nodes.clone()))
                }
            } else {
                None
            };

            if let Some(new_edge) = new_edge {
                /* println!(
                    "Replacing edge {} - {} with {} - {}.",
                    a.iter().join("/"),
                    b.iter().join("/"),
                    new_edge.0.iter().join("/"),
                    new_edge.1.iter().join("/")
                ); */

                new_edges.push(new_edge);
                false
            } else {
                true
            }
        });

        g.append(&mut new_edges)
    }
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day25_input.txt")?;
    find_mincut_product_parallelized(&input, 3);

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Graph> {
    let input = read_to_string(input_path).context("Could not open file!")?;

    let parsed_input: Vec<Result<(String, Vec<String>)>> = input
        .lines()
        .map(|l| {
            if let Some(s) = l.split_once(':') {
                Ok((
                    s.0.to_string(),
                    s.1.trim().split(' ').map(|d| d.to_string()).collect(),
                ))
            } else {
                Err(anyhow!("Could not parse line: {}", l))
            }
        })
        .collect();
    let parsed_input: Result<Vec<(String, Vec<String>)>> = parsed_input.into_iter().collect();

    // Build graph edges A-B such that A<B
    let mut edges = Graph::new();
    for (source, dests) in parsed_input?.iter() {
        for dest in dests {
            let mut source_set = BTreeSet::new();
            source_set.insert(source.to_string());
            let mut dest_set = BTreeSet::new();
            dest_set.insert(dest.to_string());

            if source < dest {
                edges.push((source_set, dest_set));
            } else {
                edges.push((dest_set, source_set));
            }
        }
    }

    Ok(edges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = read_input_file("../inputs/day25_example.txt").unwrap();
        assert_eq!(find_mincut_product(&input, 3), 54);
    }
}
