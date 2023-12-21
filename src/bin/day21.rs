use std::collections::HashSet;

enum Direction { North, East, South, West }

struct Bounds { width: isize, height: isize }
#[derive(PartialEq, Eq, Hash, Clone)]
struct Coord { x: isize, y: isize }
impl Coord {
    fn bounded_step(&self, dir: &Direction, bounds: &Bounds) -> Option<Coord> {
        match dir {
            Direction::North => if self.y > 0 { Some(Coord { x: self.x, y: self.y - 1 }) } else { None },
            Direction::East => if self.x < bounds.width - 1 { Some(Coord { x: self.x + 1, y: self.y }) } else { None },
            Direction::South => if self.y < bounds.height - 1 { Some(Coord { x: self.x, y: self.y + 1 }) } else { None },
            Direction::West => if self.x > 0 { Some(Coord { x: self.x - 1, y: self.y }) } else { None },
        }
    }

    fn step(&self, dir: &Direction) -> Coord {
        match dir {
            Direction::North => Coord { x: self.x, y: self.y - 1 },
            Direction::East => Coord { x: self.x + 1, y: self.y },
            Direction::South => Coord { x: self.x, y: self.y + 1 },
            Direction::West => Coord { x: self.x - 1, y: self.y },
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
                    start = Some(Coord { x: x as isize, y: y as isize });
                    row.push('.');
                } else {
                    row.push(char);
                }
            }
            width = row.len();
            grid.push(row);
        }
        (Map { bounds: Bounds { width: width as isize, height: grid.len() as isize }, grid }, start.unwrap())
    }

    fn get(&self, coord: &Coord) -> Option<char> {
        if coord.x < self.bounds.width && coord.y < self.bounds.height {
            Some(self.grid[coord.y as usize][coord.x as usize])
        } else {
            None
        }
    }

    fn get_wrapped(&self, coord: &Coord) -> char {
        let y = coord.y.rem_euclid(self.bounds.height) as usize;
        let x = coord.x.rem_euclid(self.bounds.width) as usize;
        self.grid[y][x]
    }

    fn is_open(&self, coord: &Coord, allow_wrapping: bool) -> bool {
        let char = if allow_wrapping { Some(self.get_wrapped(coord)) } else { self.get(coord) };
        char.map(|c| c == '.').unwrap_or(false)
    }

    fn try_move(&self, start: &Coord, dir: &Direction, allow_wrapping: bool) -> Option<Coord> {
        if allow_wrapping {
            Some(start.step(dir)).filter(|c| self.is_open(c, allow_wrapping))
        } else {
            start.bounded_step(dir, &self.bounds).filter(|c| self.is_open(c, allow_wrapping))
        }
    }
}

fn find_next_steps(starts: HashSet<Coord>, map: &Map, allow_wrapping: bool) -> HashSet<Coord> {
    let mut next_steps = HashSet::new();
    for start in starts {
        for dir in [Direction::North, Direction::East, Direction::South, Direction::West] {
            if let Some(next_step) = map.try_move(&start, &dir, allow_wrapping) {
                next_steps.insert(next_step);
            }
        }
    }
    next_steps
}

fn find_positions_after(start: &Coord, steps: usize, map: &Map, allow_wrapping: bool) -> HashSet<Coord> {
    let mut positions = HashSet::new();
    positions.insert(start.clone());
    for i in 0..steps {
        if i % 500 == 0 {
            println!("{} steps completed ({} %)", i, i * 100 / steps);
        }
        positions = find_next_steps(positions, map, allow_wrapping);
    }
    positions
}

fn find_num_positions_after(start: &Coord, steps: usize, map: &Map, allow_wrapping: bool) -> usize {
    find_positions_after(start, steps, map, allow_wrapping).len()
}

fn main() {
    let input = include_str!("../../input/day21");
    let (map, start) = Map::parse(input);
    println!("Part 1: {}", find_num_positions_after(&start, 64, &map, false));
    println!("Part 2: {}", find_num_positions_after(&start, 26501365, &map, false));
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
        assert_eq!(find_num_positions_after(&start, 1, &map, false), 2);
        assert_eq!(find_num_positions_after(&start, 2, &map, false), 4);
        assert_eq!(find_num_positions_after(&start, 3, &map, false), 6);
        assert_eq!(find_num_positions_after(&start, 6, &map, false), 16);
    }

    #[test]
    fn test_find_num_positions_after_wrapping() {
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
        // assert_eq!(find_num_positions_after(&start, 6, &map, true), 16);
        // assert_eq!(find_num_positions_after(&start, 10, &map, true), 50);
        // assert_eq!(find_num_positions_after(&start, 50, &map, true), 1594);
        // assert_eq!(find_num_positions_after(&start, 100, &map, true), 6536);
        assert_eq!(find_num_positions_after(&start, 500, &map, true), 167004);
        // assert_eq!(find_num_positions_after(&start, 1000, &map, true), 668697);
        // assert_eq!(find_num_positions_after(&start, 5000, &map, true), 16733044);
    }
}