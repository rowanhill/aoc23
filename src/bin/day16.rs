use std::collections::HashSet;

struct Tile(u8);
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Beam {
    coord: Coord,
    direction: Direction,
}

impl Tile {
    fn next_directions(&self, direction: &Direction) -> (Direction, Option<Direction>) {
        match self.0 {
            b'/' => match direction {
                Direction::North => (Direction::East, None),
                Direction::South => (Direction::West, None),
                Direction::East => (Direction::North, None),
                Direction::West => (Direction::South, None),
            },
            b'\\' => match direction {
                Direction::North => (Direction::West, None),
                Direction::South => (Direction::East, None),
                Direction::East => (Direction::South, None),
                Direction::West => (Direction::North, None),
            },
            b'-' => match direction {
                Direction::North | Direction::South => (Direction::West, Some(Direction::East)),
                _ => (*direction, None)
            },
            b'|' => match direction {
                Direction::East | Direction::West => (Direction::North, Some(Direction::South)),
                _ => (*direction, None)
            },
            b'.' => (*direction, None),
            _ => panic!("Unknown tile: {}", self.0),
        }
    }
}

impl Coord {
    fn step(&self, direction: &Direction, limits: &Coord) -> Option<Self> {
        match direction {
            Direction::North => if self.y > 0 {
                Some(Coord {
                    x: self.x,
                    y: self.y - 1,
                })
            } else {
                None
            },
            Direction::South => if self.y < limits.y {
                Some(Coord {
                    x: self.x,
                    y: self.y + 1,
                })
            } else {
                None
            },
            Direction::East => if self.x < limits.x {
                Some(Coord {
                    x: self.x + 1,
                    y: self.y,
                })
            } else {
                None
            },
            Direction::West => if self.x > 0 {
                Some(Coord {
                    x: self.x - 1,
                    y: self.y,
                })
            } else {
                None
            },
        }
    }
}

impl Beam {
    fn step(&self, tile: &Tile, limits: &Coord) -> (Option<Beam>, Option<Beam>) {
        let (dir_a, dir_b) = tile.next_directions(&self.direction);
        let beam_a = self.coord.step(&dir_a, limits).map(|coord| Beam { coord, direction: dir_a });
        let beam_b = dir_b.and_then(|dir| self.coord.step(&dir, limits).map(|coord| Beam { coord, direction: dir }));
        (beam_a, beam_b)
    }
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input.lines().map(|line| {
        line.bytes().map(Tile).collect()
    }).collect()
}

fn num_energised_tiles(tiles: &Vec<Vec<Tile>>, initial_beam: Beam) -> usize {
    let mut beams = vec![initial_beam];

    let limits = Coord {
        x: tiles[0].len() - 1,
        y: tiles.len() - 1,
    };

    let mut visited = HashSet::new();

    // Step all the beams until they leave the grid or enter a loop
    while let Some(beam) = beams.pop() {
        let tile = &tiles[beam.coord.y][beam.coord.x];
        if visited.contains(&beam) {
            continue;
        }
        visited.insert(beam);
        let (beam_a, beam_b) = beam.step(tile, &limits);
        beams.extend(beam_a);
        beams.extend(beam_b);
    }

    // Count the number of energised tiles
    visited.into_iter().map(|b| b.coord).collect::<HashSet<_>>().len()
}

fn part1(tiles: &Vec<Vec<Tile>>) -> usize {
    num_energised_tiles(tiles, Beam {
        coord: Coord {
            x: 0,
            y: 0,
        },
        direction: Direction::East,
    })
}

fn part2(tiles: &Vec<Vec<Tile>>) -> usize {
    let limits = Coord {
        x: tiles[0].len() - 1,
        y: tiles.len() - 1,
    };

    (0..=limits.x).flat_map(|x| vec![
        Beam { coord: Coord { x, y: 0 }, direction: Direction::South },
        Beam { coord: Coord { x, y: limits.y }, direction: Direction::North },
    ]).chain((0..=limits.y).flat_map(|y| vec![
        Beam { coord: Coord { x: 0, y }, direction: Direction::East },
        Beam { coord: Coord { x: limits.x, y }, direction: Direction::West },
    ])).map(|initial_beam| num_energised_tiles(tiles, initial_beam))
    .max().unwrap()
}

fn main() {
    let input = include_str!("../../input/day16");
    let tiles = parse(input);
    println!("Part 1: {}", part1(&tiles));
    println!("Part 2: {}", part2(&tiles));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1() {
        let tiles = parse(EXAMPLE);
        assert_eq!(part1(&tiles), 46);
    }

    #[test]
    fn test_part2() {
        let tiles = parse(EXAMPLE);
        assert_eq!(part2(&tiles), 51);
    }
}