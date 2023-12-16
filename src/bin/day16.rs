use std::collections::HashSet;

#[derive(Clone)]
struct Tile {
    char: u8,
    is_energised: bool,
}
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
    fn next_directions(&self, direction: &Direction) -> Vec<Direction> {
        match self.char {
            b'/' => match direction {
                Direction::North => vec![Direction::East],
                Direction::South => vec![Direction::West],
                Direction::East => vec![Direction::North],
                Direction::West => vec![Direction::South],
            },
            b'\\' => match direction {
                Direction::North => vec![Direction::West],
                Direction::South => vec![Direction::East],
                Direction::East => vec![Direction::South],
                Direction::West => vec![Direction::North],
            },
            b'-' => match direction {
                Direction::North | Direction::South => vec![Direction::West, Direction::East],
                _ => vec![*direction],
            },
            b'|' => match direction {
                Direction::East | Direction::West => vec![Direction::North, Direction::South],
                _ => vec![*direction],
            },
            b'.' => vec![*direction],
            _ => panic!("Unknown tile: {}", self.char),
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
    fn step(&self, tile: &mut Tile, limits: &Coord) -> Vec<Beam> {
        tile.is_energised = true;
        let directions = tile.next_directions(&self.direction);
        directions.into_iter().filter_map(|d| {
            self.coord.step(&d, limits).map(|c| Beam {
                coord: c,
                direction: d,
            })
        }).collect()
    }
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input.lines().map(|line| {
        line.bytes().map(|b| {
            Tile {
                char: b,
                is_energised: false,
            }
        }).collect()
    }).collect()
}

fn num_energised_tiles(mut tiles: Vec<Vec<Tile>>, initial_beam: Beam) -> usize {
    let mut beams = vec![initial_beam];

    let limits = Coord {
        x: tiles[0].len() - 1,
        y: tiles.len() - 1,
    };

    let mut visited = HashSet::new();

    // Step all the beams until they leave the grid or enter a loop
    while let Some(beam) = beams.pop() {
        let tile = &mut tiles[beam.coord.y][beam.coord.x];
        if visited.contains(&beam) {
            continue;
        }
        visited.insert(beam);
        beams.extend(beam.step(tile, &limits));
    }

    // Count the number of energised tiles
    tiles.iter().flatten().filter(|t| t.is_energised).count()
}

fn part1(tiles: Vec<Vec<Tile>>) -> usize {
    num_energised_tiles(tiles, Beam {
        coord: Coord {
            x: 0,
            y: 0,
        },
        direction: Direction::East,
    })
}

fn part2(tiles: Vec<Vec<Tile>>) -> usize {
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
    ])).map(|initial_beam| num_energised_tiles(tiles.clone(), initial_beam))
    .max().unwrap()
}

fn main() {
    let input = include_str!("../../input/day16");
    let tiles = parse(input);
    println!("Part 1: {}", part1(tiles.clone()));
    println!("Part 2: {}", part2(tiles));
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
        assert_eq!(part1(tiles), 46);
    }

    #[test]
    fn test_part2() {
        let tiles = parse(EXAMPLE);
        assert_eq!(part2(tiles), 51);
    }
}