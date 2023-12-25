#![feature(extract_if)]
use std::collections::HashMap;

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

fn dfs_for_bisecting_triple_cut(map: &HashMap<String, Vec<String>>, start: &String, visited: &mut Vec<String>) -> Option<usize> {
    let num_peripheral_edges = visited.iter()
        .map(|n| map[n].iter().filter(|n2| !visited.contains(*n2)).count())
        .sum::<usize>();
    if num_peripheral_edges == 3 {
        return Some(visited.len());
    }

    for neighbour in &map[start] {
        if visited.contains(neighbour) {
            continue;
        }

        visited.push(neighbour.to_string());

        if let Some(len) = dfs_for_bisecting_triple_cut(map, neighbour, visited) {
            return Some(len);
        }

        visited.pop();
    }
    None
}

fn part1(map: &HashMap<String, Vec<String>>) -> usize {
    let start = map.keys().next().unwrap().to_string();
    let mut visited = vec![start.clone()];
    let group_a_len = dfs_for_bisecting_triple_cut(map, &start, &mut visited).unwrap();
    let group_b_len = map.len() - group_a_len;
    group_a_len * group_b_len
}

fn print_graphviz(map: &HashMap<String, Vec<String>>) {
    println!("graph {{");
    for (from, tos) in map {
        for to in tos {
            println!("  {} -- {}", from, to);
        }
    }
    println!("}}");
}

fn main() {
    let input = include_str!("../../input/day25");
    let map = parse(input);
    // print_graphviz(&map);
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