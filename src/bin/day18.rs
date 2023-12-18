use std::collections::{HashMap, BTreeSet};

enum Direction { North, East, South, West }
impl Direction {
    fn parse(line: &str) -> Self {
        match line {
            "U" => Direction::North,
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            _ => panic!("Invalid direction: {}", line),
        }
    }

    fn convert(n: usize) -> Self {
        match n {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::North,
            _ => panic!("Invalid direction: {}", n),
        }
    }
}

struct Coordinate {
    x: isize,
    y: isize,
}

struct DigStep {
    direction: Direction,
    distance: isize,
}
impl DigStep {
    fn parse(line: &str) -> (Self, Self) {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let direction = Direction::parse(parts[0]);
        let distance = parts[1].parse::<isize>().unwrap();
        let colour_part = &parts[2][2..=parts[2].len()-2];
        let colour = usize::from_str_radix(colour_part, 16).unwrap_or_else(|_| panic!("Invalid colour: {}", colour_part));
        let colour_dist = colour / 16;
        let colour_dir = Direction::convert(colour % 16);
        (
            DigStep { direction, distance },
            DigStep { direction: colour_dir, distance: colour_dist as isize },
        )
    }
}

struct Lagoon {
    dig_plan: Vec<DigStep>,
}
impl Lagoon {
    fn parse(input: &str) -> (Self, Self) {
        let mut normal_dig_plan = Vec::new();
        let mut colour_dig_plan = Vec::new();
        for line in input.lines() {
            let (normal_dig_step, colour_dig_step) = DigStep::parse(line);
            normal_dig_plan.push(normal_dig_step);
            colour_dig_plan.push(colour_dig_step);
        }
        (
            Lagoon { dig_plan: normal_dig_plan },
            Lagoon { dig_plan: colour_dig_plan },
        )
    }

    fn calc_crossings_by_row(&self) -> HashMap<isize, BTreeSet<(isize, isize, bool)>> {
        // Map of row index to (an ordered set of (left, right) column indexes where the lagoon edge crosses the row)
        // Where an edge is perpendicular to the row, left and right are the same.
        // Where an edge runs along the row, it is considered to cross the row  IF one end leads north
        // and the other row leads south (otherwise the row is not crossed).
        // Horizontal edges are inclusive of the start and end points. Vertical edges are exclusive of both.
        let mut crossings_by_row = HashMap::new();

        let mut cur_coord = Coordinate { x: 0, y: 0 };

        for i in -1..(self.dig_plan.len() as isize - 1) {
            let prev_dig_step = &self.dig_plan[(i.rem_euclid(self.dig_plan.len() as isize)) as usize];
            let cur_dig_step = &self.dig_plan[((i+1).rem_euclid(self.dig_plan.len() as isize)) as usize];
            let next_dig_step = &self.dig_plan[((i+2).rem_euclid(self.dig_plan.len() as isize)) as usize];

            match &cur_dig_step.direction {
                Direction::North => {
                    let end_coord = Coordinate { x: cur_coord.x, y: cur_coord.y - cur_dig_step.distance };
                    for y in (end_coord.y+1)..cur_coord.y {
                        let crossings = crossings_by_row.entry(y).or_insert_with(BTreeSet::new);
                        crossings.insert((cur_coord.x, cur_coord.x, true));
                    }
                    cur_coord = end_coord;
                },
                Direction::South => {
                    let end_coord = Coordinate { x: cur_coord.x, y: cur_coord.y + cur_dig_step.distance };
                    for y in (cur_coord.y+1)..end_coord.y {
                        let crossings = crossings_by_row.entry(y).or_insert_with(BTreeSet::new);
                        crossings.insert((cur_coord.x, cur_coord.x, true));
                    }
                    cur_coord = end_coord;
                },
                Direction::West => {
                    let end_coord = Coordinate { x: cur_coord.x - cur_dig_step.distance, y: cur_coord.y };
                    let crossings = crossings_by_row.entry(cur_coord.y).or_insert_with(BTreeSet::new);
                    let crosses = matches!((&prev_dig_step.direction, &next_dig_step.direction), (Direction::North, Direction::North) | (Direction::South, Direction::South));
                    crossings.insert((end_coord.x, cur_coord.x, crosses));
                    cur_coord = end_coord;
                },
                Direction::East => {
                    let end_coord = Coordinate { x: cur_coord.x + cur_dig_step.distance, y: cur_coord.y };
                    let crossings = crossings_by_row.entry(cur_coord.y).or_insert_with(BTreeSet::new);
                    let crosses = matches!((&prev_dig_step.direction, &next_dig_step.direction), (Direction::North, Direction::North) | (Direction::South, Direction::South));
                    crossings.insert((cur_coord.x, end_coord.x, crosses));
                    cur_coord = end_coord;
                },
            }
        }

        crossings_by_row
    }

    fn count_squares_inside_lagoon_for_row(row_crossings: &BTreeSet<(isize, isize, bool)>) -> isize {
        let mut lagoon_size = 0;

        let mut is_inside = false;
        let mut prev_right = None;
        for &(left, right, crosses) in row_crossings {
            // If we're inside the lagoon, we need to deal with the gap between the previous crossing and this one
            if is_inside {
                if let Some(prev_right) = prev_right {
                    // Add the gap between the previous crossing and this one, exclusive of the range of both crossings
                    lagoon_size += left - (prev_right + 1);
                }
            }

            // Add the range covered by this crossing, inclusive of both ends
            lagoon_size += right - left + 1;

            if crosses {
                is_inside = !is_inside;
            }
            prev_right = Some(right);
        }

        lagoon_size
    }

    fn calc_lagoon_size(&self) -> usize {
        let crossings_by_row = self.calc_crossings_by_row();

        // Use the even-odd method to count the number of squares in each row of the lagoon and sum them
        let mut lagoon_size = 0;
        for (_, row_crossings) in crossings_by_row {
            lagoon_size += Lagoon::count_squares_inside_lagoon_for_row(&row_crossings);
        }

        lagoon_size as usize
    }
}

fn main() {
    let input = include_str!("../../input/day18");
    let (lagoon, colour_lagoon) = Lagoon::parse(input);
    println!("Part 1: {}", lagoon.calc_lagoon_size());
    println!("Part 2: {}", colour_lagoon.calc_lagoon_size());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1_example1() {
        let input = EXAMPLE_1;
        let (lagoon, _) = Lagoon::parse(input);
        assert_eq!(lagoon.calc_lagoon_size(), 62);
    }

    #[test]
    fn test_part2_example1() {
        let input = EXAMPLE_1;
        let (_, colour_lagoon) = Lagoon::parse(input);
        assert_eq!(colour_lagoon.calc_lagoon_size(), 952408144115);
    }

    #[test]
    fn test_calc_crossings_by_row_flat_rectangle() {
        let input = "R 4 (#000000)
D 1 (#000000)
L 4 (#000000)
U 1 (#000000)";
        let (lagoon, _) = Lagoon::parse(input);
        let crossings_by_row = lagoon.calc_crossings_by_row();
        assert_eq!(crossings_by_row.len(), 2);
        assert_eq!(crossings_by_row[&0], [(0, 4, false)].iter().cloned().collect());
        assert_eq!(crossings_by_row[&1], [(0, 4, false)].iter().cloned().collect());
    }

    #[test]
    fn test_calc_crossings_by_row_tall_rectangle() {
        let input = "R 4 (#000000)
D 2 (#000000)
L 4 (#000000)
U 2 (#000000)";
        let (lagoon, _) = Lagoon::parse(input);
        let crossings_by_row = lagoon.calc_crossings_by_row();
        assert_eq!(crossings_by_row.len(), 3);
        assert_eq!(crossings_by_row[&0], [(0, 4, false)].iter().cloned().collect());
        assert_eq!(crossings_by_row[&1], [(0, 0, true), (4, 4, true)].iter().cloned().collect());
        assert_eq!(crossings_by_row[&2], [(0, 4, false)].iter().cloned().collect());
    }

    #[test]
    fn test_calc_crossings_by_row_example() {
        let (lagoon, _) = Lagoon::parse(EXAMPLE_1);
        let crossings_by_row = lagoon.calc_crossings_by_row();
        assert_eq!(crossings_by_row.len(), 10);
        assert_eq!(crossings_by_row[&0], [(0, 6, false)].iter().cloned().collect());
        assert_eq!(crossings_by_row[&1], [(0, 0, true), (6, 6, true)].iter().cloned().collect());
        assert_eq!(crossings_by_row[&2], [(0, 2, true), (6, 6, true)].iter().cloned().collect());
        assert_eq!(crossings_by_row[&3], [(2, 2, true), (6, 6, true)].iter().cloned().collect());
        assert_eq!(crossings_by_row[&4], [(2, 2, true), (6, 6, true)].iter().cloned().collect());
        assert_eq!(crossings_by_row[&5], [(0, 2, true), (4, 6, true)].iter().cloned().collect());
        assert_eq!(crossings_by_row[&6], [(0, 0, true), (4, 4, true)].iter().cloned().collect());
        assert_eq!(crossings_by_row[&7], [(0, 1, true), (4, 6, true)].iter().cloned().collect());
        assert_eq!(crossings_by_row[&8], [(1, 1, true), (6, 6, true)].iter().cloned().collect());
        assert_eq!(crossings_by_row[&9], [(1, 6, false)].iter().cloned().collect());
    }

    #[test]
    fn test_count_squares_inside_lagoon_for_row_with_non_crossing_horizontal() {
        assert_eq!(Lagoon::count_squares_inside_lagoon_for_row(&[(0, 4, false)].iter().cloned().collect()), 5);
    }

    #[test]
    fn test_count_squares_inside_lagoon_for_row_with_two_vertical_crossings() {
        assert_eq!(Lagoon::count_squares_inside_lagoon_for_row(&[(0, 0, true), (4, 4, true)].iter().cloned().collect()), 5);
    }

    #[test]
    fn test_count_squares_inside_lagoon_for_row_with_two_horizontal_crossings() {
        assert_eq!(Lagoon::count_squares_inside_lagoon_for_row(&[(0, 1, true), (3, 4, true)].iter().cloned().collect()), 5);
    }
}