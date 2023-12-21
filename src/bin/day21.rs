use std::collections::HashSet;

enum Direction { North, East, South, West }

struct Bounds { width: usize, height: usize }
#[derive(PartialEq, Eq, Hash, Clone)]
struct Coord { x: usize, y: usize }
impl Coord {
    fn bounded_step(&self, dir: &Direction, bounds: &Bounds) -> Option<Coord> {
        match dir {
            Direction::North => if self.y > 0 { Some(Coord { x: self.x, y: self.y - 1 }) } else { None },
            Direction::East => if self.x < bounds.width - 1 { Some(Coord { x: self.x + 1, y: self.y }) } else { None },
            Direction::South => if self.y < bounds.height - 1 { Some(Coord { x: self.x, y: self.y + 1 }) } else { None },
            Direction::West => if self.x > 0 { Some(Coord { x: self.x - 1, y: self.y }) } else { None },
        }
    }
}

struct Map {
    bounds: Bounds,
    grid: Vec<Vec<char>>,
}
impl Map {
    fn parse(input: &str) -> (Map, Coord) {
        let mut grid = Vec::new();
        let mut start = None;
        let mut width = 0;
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, char) in line.chars().enumerate() {
                if char == 'S' {
                    start = Some(Coord { x, y });
                    row.push('.');
                } else {
                    row.push(char);
                }
            }
            width = row.len();
            grid.push(row);
        }
        (Map { bounds: Bounds { width, height: grid.len() }, grid }, start.unwrap())
    }

    fn get(&self, coord: &Coord) -> Option<char> {
        if coord.x < self.bounds.width && coord.y < self.bounds.height {
            Some(self.grid[coord.y][coord.x])
        } else {
            None
        }
    }

    fn is_open(&self, coord: &Coord) -> bool {
        self.get(coord).map(|c| c == '.').unwrap_or(false)
    }

    fn try_move(&self, start: &Coord, dir: &Direction) -> Option<Coord> {
        start.bounded_step(dir, &self.bounds).filter(|c| self.is_open(c))
    }
}

fn find_next_steps(starts: HashSet<Coord>, map: &Map) -> HashSet<Coord> {
    let mut next_steps = HashSet::new();
    for start in starts {
        for dir in [Direction::North, Direction::East, Direction::South, Direction::West] {
            if let Some(next_step) = map.try_move(&start, &dir) {
                next_steps.insert(next_step);
            }
        }
    }
    next_steps
}

fn find_positions_after(start: &Coord, steps: usize, map: &Map) -> HashSet<Coord> {
    let mut positions = HashSet::new();
    positions.insert(start.clone());
    for _ in 0..steps {
        positions = find_next_steps(positions, map);
    }
    positions
}

fn find_num_positions_after(start: &Coord, steps: usize, map: &Map) -> usize {
    find_positions_after(start, steps, map).len()
}

fn main() {
    let input = include_str!("../../input/day21");
    let (map, start) = Map::parse(input);
    println!("Part 1: {}", find_num_positions_after(&start, 64, &map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_num_positions_after() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let (map, start) = Map::parse(input);
        assert_eq!(find_num_positions_after(&start, 1, &map), 2);
        assert_eq!(find_num_positions_after(&start, 2, &map), 4);
        assert_eq!(find_num_positions_after(&start, 3, &map), 6);
        assert_eq!(find_num_positions_after(&start, 6, &map), 16);
    }
}