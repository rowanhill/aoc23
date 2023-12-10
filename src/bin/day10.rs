use std::collections::HashSet;

type Coord = (usize, usize);
type Direction = (i8, i8);

const NORTH: Direction = (0, -1);
const SOUTH: Direction = (0, 1);
const EAST: Direction = (1, 0);
const WEST: Direction = (-1, 0);

trait CoordExt {
    fn step(&self, direction: &Direction) -> Self;
}
impl CoordExt for Coord {
    fn step(&self, direction: &Direction) -> Self {
        ((self.0 as isize + direction.0 as isize) as usize, (self.1 as isize + direction.1 as isize) as usize)
    }
}

trait DirectionExt {
    fn opposite(&self) -> Self;
    fn is_opposite(&self, other: &Self) -> bool;
}
impl DirectionExt for Direction {
    fn opposite(&self) -> Self {
        (-self.0, -self.1)
    }
    fn is_opposite(&self, other: &Self) -> bool {
        self.0 == -other.0 && self.1 == -other.1
    }
}

type Input = &'static [u8];
struct Map {
    bytes: Input,
    width: usize,
    height: usize,
    start: Coord,
}

impl Map {
    fn new(bytes: Input) -> Self {
        let width = bytes.iter().position(|&b| b == b'\n').unwrap();
        let height = bytes.len() / (width + 1);
        let start = bytes.iter().position(|&b| b == b'S').unwrap();
        Self { bytes, width, height, start: (start % (width + 1), start / width) }
    }

    // Find how many steps it takes to get from the start, S, back to the start.
    // | connects N and S, - connects E and W, L connects N and E, J connects S and E, 7 connects S and W, F connects N and W.
    fn loop_length(&self) -> usize {
        self.loop_definition().len()
    }

    fn count_inside_loop(&self) -> usize {
        let loop_coords = self.loop_definition();
        let mut count_inside_loop = 0;
        let mut horiz_incoming_dir = None;
        for y in 0..self.height {
            let mut num_crossings_for_row = 0;
            for x in 0..self.width {
                let coord = (x, y);
                if loop_coords.contains(&coord) {
                    let map_char = self.get(&coord);
                    let map_char = if map_char == b'S' {
                        let available_moves = self.available_moves(&coord);
                        if available_moves.contains(&NORTH) && available_moves.contains(&SOUTH) {
                            b'|'
                        } else if available_moves.contains(&EAST) && available_moves.contains(&WEST) {
                            b'-'
                        } else if available_moves.contains(&NORTH) && available_moves.contains(&EAST) {
                            b'L'
                        } else if available_moves.contains(&NORTH) && available_moves.contains(&WEST) {
                            b'J'
                        } else if available_moves.contains(&SOUTH) && available_moves.contains(&WEST) {
                            b'7'
                        } else if available_moves.contains(&SOUTH) && available_moves.contains(&EAST) {
                            b'F'
                        } else {
                            panic!("Invalid start coord: {:?}", coord);
                        }
                    } else {
                        map_char
                    };
                    match map_char {
                        b'|' => {
                            num_crossings_for_row += 1;
                        },
                        b'L' => horiz_incoming_dir = Some(NORTH),
                        b'F' => horiz_incoming_dir = Some(SOUTH),
                        b'J' => {
                            if let Some(dir) = horiz_incoming_dir {
                                if dir == SOUTH {
                                    num_crossings_for_row += 1;
                                }
                            }
                            horiz_incoming_dir = None;
                        }
                        b'7' => {
                            if let Some(dir) = horiz_incoming_dir {
                                if dir == NORTH {
                                    num_crossings_for_row += 1;
                                }
                            }
                            horiz_incoming_dir = None;
                        },
                        _ => {}
                    }
                } else if num_crossings_for_row % 2 == 1 {
                    count_inside_loop += 1;
                }
            }
        }
        count_inside_loop
    }

    fn loop_definition(&self) -> HashSet<Coord> {
        let mut current_dir = self.available_moves(&self.start)[0];
        let mut current_coord = self.start.step(&current_dir);
        let mut loop_coords = HashSet::new();
        loop_coords.insert(self.start);
        while self.get(&current_coord) != b'S' {
            loop_coords.insert(current_coord);
            current_dir = *self.available_moves(&current_coord).iter()
                .find(|&&dir| !dir.is_opposite(&current_dir)).unwrap();
            current_coord = current_coord.step(&current_dir);
        }
        loop_coords
    }

    fn available_moves(&self, coord: &Coord) -> Vec<Direction> {
        match self.get(coord) {
            b'.' => vec![],
            b'S' => {
                vec![NORTH, SOUTH, EAST, WEST].into_iter()
                    .filter(|dir| self.is_step_in_bounds(coord, dir))
                    .filter(|dir| self.available_moves(&coord.step(dir)).contains(&dir.opposite()))
                    .collect()
            },
            b => {
                match b {
                    b'|' => [NORTH, SOUTH],
                    b'-' => [EAST, WEST],
                    b'L' => [NORTH, EAST],
                    b'J' => [NORTH, WEST],
                    b'7' => [SOUTH, WEST],
                    b'F' => [SOUTH, EAST],
                    _ => panic!("Invalid byte at {:?}", coord),
                }.into_iter()
                .filter(|dir| self.is_step_in_bounds(coord, dir))
                .collect()
            }
        }
    }

    fn is_step_in_bounds(&self, coord: &Coord, direction: &Direction) -> bool {
        (direction.0 >= 0 || coord.0 > 0) &&
            (direction.0 <= 0 || coord.0 < self.width) &&
            (direction.1 >= 0 || coord.1 > 0) &&
            (direction.1 <= 0 || coord.1 < self.height)
    }

    fn get(&self, coord: &Coord) -> u8 {
        self.bytes[coord.1 * (self.width + 1) + coord.0]
    }
}

fn part1(input: Input) -> usize {
    let map = Map::new(input);
    map.loop_length() / 2
}

fn part2(input: Input) -> usize {
    let map = Map::new(input);
    map.count_inside_loop()
}

fn main() {
    let input = include_bytes!("../../input/day10");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    const EXAMPLE_2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_1.as_bytes()), 4);
        assert_eq!(part1(EXAMPLE_2.as_bytes()), 8);
    }

    const EXAMPLE_3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_3.as_bytes()), 4);
    }
}