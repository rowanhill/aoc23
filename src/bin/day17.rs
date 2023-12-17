use std::{collections::{BinaryHeap, HashMap}, cmp::Reverse};

fn parse(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|line| line.bytes().map(|b| (b - b'0') as usize).collect()).collect()
}

fn shortest_path(grid: &[Vec<usize>], start: (usize, usize), target: (usize, usize)) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start, (None, None, None))));
    let mut min_dists = HashMap::new();
    min_dists.insert(start, 0);
    let mut prev = HashMap::new();

    while let Some(Reverse((dist, (x, y), steps))) = queue.pop() {
        let three_step_dir = if let (Some(s1), Some(s2), Some(s3)) = steps {
            if s1 == s2 && s2 == s3 {
                Some(s1)
            } else {
                None
            }
        } else {
            None
        };

        for (dx, dy) in &[(0isize, 1isize), (0, -1), (1, 0), (-1, 0)] {
            if let Some(dir) = three_step_dir {
                if dir == (*dx, *dy) {
                    continue;
                }
            }
            let nx = if *dx >= 0 { x.checked_add(*dx as usize) } else { x.checked_sub(1) };
            let ny = if *dy >= 0 { y.checked_add(*dy as usize) } else { y.checked_sub(1) };
            if let (Some(nx), Some(ny)) = (nx, ny) {
                if nx >= width || ny >= height {
                    continue;
                }
                let weight = grid[ny][nx];
                if (nx, ny) == target {
                    let mut path = vec![target, (x, y)];
                    let mut cur = (x, y);
                    while let Some(prev) = prev.get(&cur) {
                        path.push(*prev);
                        cur = *prev;
                    }
                    path.reverse();
                    println!("Path: {:?}", path);
                    return dist + weight;
                }
                if dist + weight < *min_dists.get(&(nx, ny)).unwrap_or(&usize::MAX) {
                    min_dists.insert((nx, ny), dist + weight);
                    queue.push(Reverse((dist + weight, (nx, ny), (steps.1, steps.2, Some((*dx, *dy))))));
                    prev.insert((nx, ny), (x, y));
                }
            }
        }
    }

    panic!("No path found");
}

fn main() {
    let input = include_str!("../../input/day17");
    let grid = parse(input);
    let shortest_path = shortest_path(&grid, (0, 0), (grid[0].len() - 1, grid.len() - 1));
    println!("Part 1: {}", shortest_path);
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
    fn test_shortest_path() {
        let grid = parse(EXAMPLE);
        assert_eq!(shortest_path(&grid, (0, 0), (grid[0].len() - 1, grid.len() - 1)), 102);
    }
}