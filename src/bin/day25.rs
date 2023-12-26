use std::collections::{HashMap, HashSet, VecDeque};

struct Graph{
    neighbours: HashMap<usize, Vec<usize>>,
    excluded_edges: HashSet<(usize, usize)>,
}

impl Graph {
    fn parse(input: &str) -> Self {
        let mut nodes = HashMap::new();
        let mut neighbours = HashMap::new();
        for line in input.lines() {
            let (from, tos) = line.split_once(": ").unwrap();
            let len = nodes.len();
            let from_index = *nodes.entry(from).or_insert(len);
            for to in tos.split(' ') {
                let len = nodes.len();
                let to_index = *nodes.entry(to).or_insert(len);
                neighbours.entry(to_index).or_insert_with(Vec::new).push(from_index);
                neighbours.entry(from_index).or_insert_with(Vec::new).push(to_index);
            }
        }
        Graph { neighbours, excluded_edges: HashSet::new() }
    }

    fn add_excluded_edge(&mut self, a: usize, b: usize) {
        let low = a.min(b);
        let high = a.max(b);
        self.excluded_edges.insert((low, high));
    }

    fn is_edge_excluded(&self, a: usize, b: usize) -> bool {
        let low = a.min(b);
        let high = a.max(b);
        self.excluded_edges.contains(&(low, high))
    }

    fn remove_excluded_edge(&mut self, a: usize, b: usize) {
        let low = a.min(b);
        let high = a.max(b);
        self.excluded_edges.remove(&(low, high));
    }

    fn get_neighbours<'a>(&'a self, node: &'a usize) -> impl Iterator<Item=&usize> + 'a {
        self.neighbours[node].iter()
            .filter(|&&n| !self.is_edge_excluded(n, *node))
    }

    fn path(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut predecessors = HashMap::new();
        
        queue.push_back(from);
        while let Some(node) = queue.pop_front() {
            if !visited.insert(node) {
                continue;
            }
            if node == to {
                break;
            }
            for next in self.get_neighbours(&node) {
                if !visited.contains(next) {
                    queue.push_back(*next);
                    predecessors.insert(*next, node);
                }
            }
        }

        let mut node = to;
        let mut path = Vec::new();
        while node != from {
            if !predecessors.contains_key(&node) {
                return None;
            }
            path.push(node);
            node = predecessors[&node];
        }
        path.push(from);
        path.reverse();
        Some(path)
    }

    fn three_exclusions_can_bisect(
        &mut self,
        start: usize,
        end: usize,
        previously_seen_edges: &HashSet<(usize, usize)>,
    ) -> bool {
        let path = self.path(start, end);

        if path.is_none() {
            // We've split the graph in two. If we did so with 3 exclusions, we've found the right ones.
            // Otherwise, we've not found a solution
            return self.excluded_edges.len() == 3;
        }

        if self.excluded_edges.len() == 3 {
            // We've removed three edges, but the graph is still connected. This is not the right solution.
            return false;
        }

        // Create all the edges traversed in the path
        let path = path.unwrap();
        let edges = path.windows(2);

        let mut new_previously_seen_edges = previously_seen_edges.clone();
        new_previously_seen_edges.extend(edges.clone().map(|e| (e[0], e[1])));

        // For each of the edges, see if removing it will split the graph in two
        for edge in edges {
            if previously_seen_edges.contains(&(edge[0], edge[1])) {
                // If we saw this edge in a previous iteration's path, it can't be part of the solution
                // (because that would mean the previous path contained multiple edges traversing the two subgraphs
                // and we've assumed that the path moves between the two subgraphs)
                continue;
            }

            // Pick this edge to remove and recurse
            self.add_excluded_edge(edge[0], edge[1]);
            if self.three_exclusions_can_bisect(start, end, &new_previously_seen_edges) {
                return true;
            }
            self.remove_excluded_edge(edge[0], edge[1]);
        }

        // If we didn't find a solution, there isn't one
        false
    }

    fn sizes_of_bisected_subgraphs(&mut self) -> (usize, usize) {
        for i in 1..self.neighbours.len() {
            if self.three_exclusions_can_bisect(0, i, &HashSet::new()) {
                let subgraph_a_size = self.graph_len_excluding_edges();
                let subgraph_b_size = self.neighbours.len() - subgraph_a_size;
                return (subgraph_a_size, subgraph_b_size);
            }
        }
        panic!("No solution found");
    }

    fn graph_len_excluding_edges(&self) -> usize {
        let start = *self.neighbours.keys().next().unwrap();

        let mut visited = HashSet::new();
        let mut queue = vec![start];
        while let Some(node) = queue.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            for next in self.get_neighbours(&node) {
                if visited.contains(next) {
                    continue;
                }
                queue.push(*next);
            }
        }
        visited.len()
    }
}

fn main() {
    let input = include_str!("../../input/day25");
    let mut graph = Graph::parse(input);
    let (a, b) = graph.sizes_of_bisected_subgraphs();
    println!("Part 1: {}", a * b);
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
        let mut graph = Graph::parse(input);
        let (a, b) = graph.sizes_of_bisected_subgraphs();
        assert_eq!(a * b, 54);
    }
}