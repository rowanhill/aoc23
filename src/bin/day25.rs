#![feature(extract_if)]
use std::{collections::{HashMap, BinaryHeap, HashSet}, cmp::Reverse};

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (from, tos) = line.split_once(": ").unwrap();
        for to in tos.split(' ') {
            map.entry(to.to_string()).or_insert_with(Vec::new).push(from.to_string());
            map.entry(from.to_string()).or_insert_with(Vec::new).push(to.to_string());
        }
    }
    map
}

fn neighbours_excluding_edges(map: &HashMap<String, Vec<String>>, node: &String, exclusions: &HashSet<(String, String)>) -> Vec<String> {
    map[node].iter()
        .filter(|n|
            !exclusions.contains(&(n.to_string(), node.to_string())) &&
            !exclusions.contains(&(node.to_string(), n.to_string()))
        )
        .map(|n| n.to_string())
        .collect()
}

fn shortest_path(map: &HashMap<String, Vec<String>>, from: &String, to: &String, exclusions: &HashSet<(String, String)>) -> Option<Vec<String>> {
    let mut visited = vec![from.to_string()];
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), vec![from.to_string()]));
    while let Some((Reverse(len), path)) = queue.pop() {
        let node = path.last().unwrap();
        if node == to {
            return Some(path);
        }
        for next in &neighbours_excluding_edges(map, node, exclusions) {
            if !visited.contains(next) {
                visited.push(next.to_string());
                let mut new_path = path.clone();
                new_path.push(next.to_string());
                queue.push((Reverse(len + 1), new_path));
            }
        }
    }
    None
}

fn find_wires_to_cut(
    map: &HashMap<String, Vec<String>>,
    exclusions: &HashSet<(String, String)>,
    start: &String,
    end: &String,
    previously_seen: &HashSet<(String, String)>,
) -> Option<HashSet<(String, String)>> {
    let path = shortest_path(map, start, end, exclusions);

    if path.is_none() {
        // We've split the graph in two. If we did so with 3 exclusions, we've found the right ones.
        // Otherwise, we've not found a solution
        if exclusions.len() == 3 {
            return Some(exclusions.clone());
        } else {
            return None;
        }
    }

    if exclusions.len() == 3 {
        // We've removed three edges, but the graph is still connected. This is not the right solution.
        return None;
    }

    // Create all the edges traversed in the path
    let path = path.unwrap();
    let edges = path.iter().zip(path.iter().skip(1)).map(|(a, b)| (a.to_string(), b.to_string())).collect::<HashSet<_>>();

    // For each of the edges, see if removing it will split the graph in two
    for edge in &edges {
        if previously_seen.contains(edge) {
            // If we saw this edge in a previous iteration's path, it can't be part of the solution
            // (because that would mean the previous path contained multiple edges traversing the two subgraphs
            // and we've assumed that the path moves between the two subgraphs)
            continue;
        }

        // Pick this edge to remove and recurse
        let mut new_exclusions = exclusions.clone();
        new_exclusions.insert(edge.clone());
        let mut new_previously_seen = previously_seen.clone();
        new_previously_seen.extend(edges.clone());
        let found = find_wires_to_cut(map, &new_exclusions, start, end, previously_seen);
        if found.is_some() {
            return found;
        }
    }

    // If we didn't find a solution, there isn't one
    None
}

fn graph_len_excluding_edges(map: &HashMap<String, Vec<String>>, exclusions: &HashSet<(String, String)>) -> usize {
    let start = map.keys().next().unwrap();

    let mut visited = HashSet::new();
    let mut queue = vec![start.to_string()];
    while let Some(node) = queue.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node.to_string());
        for next in &neighbours_excluding_edges(map, &node, exclusions) {
            if visited.contains(next) {
                continue;
            }
            queue.push(next.to_string());
        }
    }
    visited.len()
}

fn part1(map: &HashMap<String, Vec<String>>) -> usize {
    let mut nodes = map.keys();
    let start = nodes.next().unwrap();
    for end in nodes {
        let exclusions = find_wires_to_cut(map, &HashSet::new(), start, end, &HashSet::new());
        if let Some(exclusions) = exclusions {
            let subgraph_a_size = graph_len_excluding_edges(map, &exclusions);
            let subgraph_b_size = map.len() - subgraph_a_size;
            return subgraph_a_size * subgraph_b_size;
        }
    }
    panic!("No solution found");
}

fn main() {
    let input = include_str!("../../input/day25");
    let map = parse(input);
    println!("Part 1: {}", part1(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        let map = parse(input);
        assert_eq!(part1(&map), 54);
    }
}