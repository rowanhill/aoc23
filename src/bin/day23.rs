use std::collections::HashSet;

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

enum Direction { North, East, South, West }

struct Forest {
    bytes: &'static [u8],
    width: usize,
    height: usize,
}
impl Forest {
    fn new(bytes: &'static [u8]) -> Self {
        let width = bytes.iter().position(|&b| b == b'\n').unwrap();
        let height = (bytes.len() + 1) / (width + 1);
        Self { bytes, width, height }
    }

    fn get(&self, coord: &Coord) -> Option<u8> {
        if coord.x >= self.width || coord.y >= self.height {
            None
        } else {
            Some(self.bytes[coord.y * (self.width + 1) + coord.x])
        }
    }

    fn get_permitted_neighbours(&self, coord: &Coord) -> Vec<Coord> {
        use Direction::*;
        let mut neighbours = Vec::new();
        let cur_byte = self.get(coord).unwrap();

        for (dir, slope) in [(North, b'^'), (East, b'>'), (South, b'v'), (West, b'<')] {
            if cur_byte == b'.' || cur_byte == slope {
                if let Some(next_coord) = coord.step(&dir) {
                    if let Some(next_byte) = self.get(&next_coord) {
                        if next_byte != b'#' {
                            neighbours.push(next_coord);
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

    fn find_longest_path_len_dfs(&self, start: &Coord, target: &Coord, visited: &mut HashSet<Coord>) -> Option<usize> {
        if start == target {
            return Some(visited.len() - 1);
        }

        self.get_permitted_neighbours(start).into_iter()
            .filter_map(|n| {
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
}

fn main() {
    let input = include_bytes!("../../input/day23");
    let forest = Forest::new(input);
    println!("Part 1: {:?}", forest.find_longest_path_len());
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
        assert_eq!(forest.find_longest_path_len().unwrap(), 94);
    }

    #[test]
    fn test_part1_mini() {
        let input = b"#.##
#..#
##.#";
        let forest = Forest::new(input);
        assert_eq!(forest.find_longest_path_len().unwrap(), 3);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(""), 0);
    // }
}