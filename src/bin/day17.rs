use core::panic;
use std::{collections::{BinaryHeap, HashMap}, cmp::Reverse};

fn parse(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|line| line.bytes().map(|b| (b - b'0') as usize).collect()).collect()
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Clone)]
enum Direction { North, South, East, West }
impl Direction {
    fn orthogonals(&self) -> [Direction; 2] {
        match self {
            Direction::North | Direction::South => [Direction::East, Direction::West],
            Direction::East | Direction::West => [Direction::North, Direction::South],
        }
    }
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}
struct Bounds {
    width: usize,
    height: usize,
}

impl Coordinate {
    fn step(&self, dir: &Direction, bounds: &Bounds) -> Option<Coordinate> {
        match dir {
            Direction::North => if self.y > 0 { Some(Coordinate{x: self.x, y: self.y - 1}) } else { None },
            Direction::South => if self.y < bounds.height - 1 { Some(Coordinate{x: self.x, y: self.y + 1}) } else { None },
            Direction::East => if self.x < bounds.width - 1 { Some(Coordinate{x: self.x + 1, y: self.y}) } else { None },
            Direction::West => if self.x > 0 { Some(Coordinate{x: self.x - 1, y: self.y}) } else { None },
        }
    
    }
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Clone)]
struct Crucible {
    pos: Coordinate, // Coordinate of the crucible
    dir: Direction, // Direction the crucible is facing
    steps: usize, // Number of steps the crucible has taken since turning to its current direction
}

#[derive(Ord, PartialOrd, PartialEq, Eq)]
struct SearchState {
    heat_loss: usize, // Heat loss accumulated in moving crucible into Crucible state
    crucible: Crucible,
}

fn shortest_path(grid: &[Vec<usize>], min_steps: usize, max_steps: usize) -> usize {
    use Direction::*;

    let bounds = Bounds { width: grid[0].len(), height: grid.len() };
    let target = Coordinate { x: bounds.width - 1, y: bounds.height - 1 };

    // Minimum heat loss found so far for a given Crucible
    let mut min_heat_losses = HashMap::new();

    // Min-heap queue of SearchStates
    let mut queue = BinaryHeap::new();

    // Start in the top-left, facing either east or south
    let start = Coordinate { x: 0, y: 0 };
    queue.push(Reverse(SearchState { heat_loss: 0, crucible: Crucible { pos: start.clone(), dir: East, steps: 0 } }));
    queue.push(Reverse(SearchState { heat_loss: 0, crucible: Crucible { pos: start.clone(), dir: South, steps: 0 } }));

    while let Some(Reverse(search_state)) = queue.pop() {
        let min_heat_loss_to_search_state = min_heat_losses.get(&search_state.crucible).cloned().unwrap_or(usize::MAX);
        if min_heat_loss_to_search_state <= search_state.heat_loss {
            // Another branch has already found a lower-or-equal heat loss to this crucible state, so ignore this one
            continue;
        }

        if search_state.crucible.pos == target {
            // We've reached the target, so return the heat loss
            return search_state.heat_loss;
        }

        // This is a new minimum heat loss to this crucible state, so update the min_dists map
        min_heat_losses.insert(search_state.crucible.clone(), search_state.heat_loss);

        // Explore forwards (if possible)
        if search_state.crucible.steps < max_steps {
            // We can take another step in the same direction
            if let Some(next_pos) = search_state.crucible.pos.step(&search_state.crucible.dir, &bounds) {
                let next_heat_loss = search_state.heat_loss + grid[next_pos.y][next_pos.x];
                let next_crucible = Crucible { pos: next_pos, dir: search_state.crucible.dir.clone(), steps: search_state.crucible.steps + 1 };
                let next_search_state = SearchState { heat_loss: next_heat_loss, crucible: next_crucible };
                queue.push(Reverse(next_search_state));
            }
        }

        // Explore orthogonally (if possible)
        if search_state.crucible.steps >= min_steps {
            // We can turn and take a step in a new direction
            for next_dir in search_state.crucible.dir.orthogonals() {
                if let Some(next_pos) = search_state.crucible.pos.step(&next_dir, &bounds) {
                    let next_heat_loss = search_state.heat_loss + grid[next_pos.y][next_pos.x];
                    let next_crucible = Crucible { pos: next_pos, dir: next_dir, steps: 1 }; // We've already taken one step in this direction
                    let next_search_state = SearchState { heat_loss: next_heat_loss, crucible: next_crucible };
                    queue.push(Reverse(next_search_state));
                }
            }
        }
    }

    panic!("No path found");
}

fn main() {
    let input = include_str!("../../input/day17");
    let grid = parse(input);
    println!("Part 1: {}", shortest_path(&grid, 0, 3));
    println!("Part 2: {}", shortest_path(&grid, 4, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_shortest_path_part_1() {
        let grid = parse(EXAMPLE);
        assert_eq!(shortest_path(&grid, 0, 3), 102);
    }

    #[test]
    fn test_shortest_path_part_2() {
        let grid = parse(EXAMPLE);
        assert_eq!(shortest_path(&grid, 4, 10), 94);
    }
}