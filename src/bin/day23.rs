use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord { x: usize, y: usize }
impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn step(&self, dir: &Direction) -> Option<Self> {
        match dir {
            Direction::North => {
                if self.y == 0 {
                    None
                } else {
                    Some(Self::new(self.x, self.y - 1))
                }
            },
            Direction::East => {
                Some(Self::new(self.x + 1, self.y))
            },
            Direction::South => {
                Some(Self::new(self.x, self.y + 1))
            },
            Direction::West => {
                if self.x == 0 {
                    None
                } else {
                    Some(Self::new(self.x - 1, self.y))
                }
            },
        }
    }
}

#[derive(PartialEq, Eq)]
enum Direction { North, East, South, West }
impl Direction {
    fn inverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

struct Forest {
    bytes: &'static [u8],
    width: usize,
    height: usize,
    ignore_slopes: bool,
}
impl Forest {
    fn new(bytes: &'static [u8]) -> Self {
        let width = bytes.iter().position(|&b| b == b'\n').unwrap();
        let height = (bytes.len() + 1) / (width + 1);
        Self { bytes, width, height, ignore_slopes: false }
    }

    fn get(&self, coord: &Coord) -> Option<u8> {
        if coord.x >= self.width || coord.y >= self.height {
            None
        } else {
            Some(self.bytes[coord.y * (self.width + 1) + coord.x])
        }
    }

    fn get_permitted_neighbours(&self, coord: &Coord) -> Vec<(Direction, Coord)> {
        use Direction::*;
        let mut neighbours = Vec::new();
        let cur_byte = self.get(coord).unwrap();

        for (dir, slope) in [(North, b'^'), (East, b'>'), (South, b'v'), (West, b'<')] {
            if (self.ignore_slopes && cur_byte != b'#') || (!self.ignore_slopes && (cur_byte == b'.' || cur_byte == slope)) {
                if let Some(next_coord) = coord.step(&dir) {
                    if let Some(next_byte) = self.get(&next_coord) {
                        if next_byte != b'#' {
                            neighbours.push((dir, next_coord));
                        }
                    }
                }
            }
        }

        neighbours
    }

    fn find_longest_path_len(&self) -> Option<usize> {
        let start = Coord::new(1, 0);
        let target = Coord::new(self.width - 2, self.height - 1);
        let mut visited = HashSet::new();
        visited.insert(start);
        self.find_longest_path_len_dfs(&start, &target, &mut visited)
    }

    fn find_longest_path_len_dfs(
        &self,
        start: &Coord,
        target: &Coord,
        visited: &mut HashSet<Coord>,
    ) -> Option<usize> {
        if start == target {
            return Some(visited.len() - 1);
        }

        self.get_permitted_neighbours(start).into_iter()
            .filter_map(|(_, n)| {
                if visited.insert(n) {
                    let l = self.find_longest_path_len_dfs(&n, target, visited);
                    visited.remove(&n);
                    l
                } else {
                    None
                }
            })
            .max()
    }

    fn simplify_graph(&self) -> HashMap<Coord, HashSet<(Coord, usize)>> {
        use Direction::*;

        let mut queue = Vec::new();
        queue.push((Coord::new(1, 0), South));

        let mut visited = HashSet::new();

        let mut graph = HashMap::new();

        while let Some((coord, dir)) = queue.pop() {
            let mut cur_coord = coord.step(&dir).unwrap();
            let mut steps = 1;
            let mut neighbours = self.get_permitted_neighbours(&cur_coord).into_iter()
                .filter(|(d, _)| *d != dir.inverse())
                .collect::<Vec<_>>();
            while neighbours.len() == 1 {
                steps += 1;
                let (next_dir, next_coord) = neighbours.pop().unwrap();
                cur_coord = next_coord;
                neighbours = self.get_permitted_neighbours(&cur_coord).into_iter()
                    .filter(|(d, _)| *d != next_dir.inverse())
                    .collect::<Vec<_>>();
            }
            graph.entry(coord).or_insert_with(HashSet::new).insert((cur_coord, steps));
            graph.entry(cur_coord).or_insert_with(HashSet::new).insert((coord, steps));
            if !visited.contains(&cur_coord) {
                for (d, _) in neighbours {
                    queue.push((cur_coord, d));
                }
                visited.insert(cur_coord);
            }
        }

        graph
    }

    fn find_longest_path_len_simplified(&self) -> Option<usize> {
        let graph = self.simplify_graph();
        let start = Coord::new(1, 0);
        let target = Coord::new(self.width - 2, self.height - 1);
        let mut visited = HashSet::new();
        visited.insert(start);
        self.find_longest_path_len_dfs_simplified(&graph, &start, &target, &mut visited)
    }

    fn find_longest_path_len_dfs_simplified(
        &self,
        graph: &HashMap<Coord, HashSet<(Coord, usize)>>,
        start: &Coord,
        target: &Coord,
        visited: &mut HashSet<Coord>,
    ) -> Option<usize> {
        if start == target {
            return Some(0);
        }

        graph[start].iter()
            .filter_map(|(n, steps)| {
                if visited.insert(*n) {
                    let l = self.find_longest_path_len_dfs_simplified(graph, n, target, visited);
                    visited.remove(n);
                    l.map(|l| l + steps)
                } else {
                    None
                }
            })
            .max()
    }
}

fn main() {
    let input = include_bytes!("../../input/day23");
    let mut forest = Forest::new(input);
    println!("Part 1: {:?}", forest.find_longest_path_len());
    forest.ignore_slopes = true;
    println!("Part 2: {:?}", forest.find_longest_path_len_simplified());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1() {
        let forest = Forest::new(EXAMPLE);
        assert_eq!(forest.find_longest_path_len(), Some(94));
    }

    #[test]
    fn test_part1_mini() {
        let input = b"#.##
#..#
##.#";
        let forest = Forest::new(input);
        assert_eq!(forest.find_longest_path_len(), Some(3));
    }

    #[test]
    fn test_part2() {
        let mut forest = Forest::new(EXAMPLE);
        forest.ignore_slopes = true;
        assert_eq!(forest.find_longest_path_len_simplified(), Some(154));
    }
}