use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord { x: isize, y: isize, z: isize }
impl Coord {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        Self { x, y, z }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Brick { label: String, from: Coord, to: Coord }
impl Brick {
    fn x_range(&self) -> std::ops::RangeInclusive<isize> {
        self.from.x.min(self.to.x)..=self.from.x.max(self.to.x)
    }
    fn y_range(&self) -> std::ops::RangeInclusive<isize> {
        self.from.y.min(self.to.y)..=self.from.y.max(self.to.y)
    }

    fn bottom(&self) -> isize {
        self.from.z.min(self.to.z)
    }
    fn top(&self) -> isize {
        self.from.z.max(self.to.z)
    }

    fn drop(&mut self, delta: isize) {
        self.from.z -= delta;
        self.to.z -= delta;
    }
}

struct Snapshot {
    bricks: Vec<Brick>,
    bricks_by_xy: HashMap<(isize, isize), Vec<usize>>,
    supported_by: HashMap<usize, HashSet<usize>>, // Indexes of bricks supporting each brick
    supports: HashMap<usize, HashSet<usize>>, // Indexes of bricks supported by each brick
}
impl Snapshot {
    fn parse(input: &str) -> Self {
        let mut bricks = Vec::new();
        let mut bricks_by_xy = HashMap::new();
        for line in input.lines() {
            let (from_str, to_str) = line.split_once('~').unwrap();
            let from = Coord::parse(from_str);
            let to = Coord::parse(to_str);
            let brick = Brick { label: num_to_alpha(bricks.len()), from, to };
            for x in brick.x_range() {
                for y in brick.y_range() {
                    bricks_by_xy.entry((x, y)).or_insert_with(Vec::new).push(bricks.len());
                }
            }
            bricks.push(brick);
        }
        Snapshot { bricks, bricks_by_xy, supported_by: HashMap::new(), supports: HashMap::new() }
    }

    fn settle(&mut self) {
        let mut brick_indexes_in_z_order = (0..self.bricks.len()).collect::<Vec<_>>();
        brick_indexes_in_z_order.sort_by_key(|&i| self.bricks[i].bottom());

        for brick_index in brick_indexes_in_z_order {
            let brick = &self.bricks[brick_index];
            // Skip bricks that are already settled
            if brick.bottom() <= 1 {
                // println!("Brick {} settled at z={}-{}", brick.label, brick.bottom(), brick.top());
                continue;
            }

            // Find the highest z value < the lowest z value of the brick within the same x,y
            let mut highest_z = 0;
            let mut supporters = HashSet::new();
            for x in brick.x_range() {
                for y in brick.y_range() {
                    if let Some(other_brick_indices) = self.bricks_by_xy.get(&(x, y)) {
                        for &other_brick_index in other_brick_indices {
                            let other_brick = &self.bricks[other_brick_index];
                            if other_brick.top() < brick.bottom() {
                                if highest_z < other_brick.top() {
                                    highest_z = other_brick.top();
                                    supporters.clear();
                                }
                                if highest_z <= other_brick.top() {
                                    supporters.insert(other_brick_index);
                                }
                            }
                        }
                    }
                }
            }
            for supporter in &supporters {
                self.supports.entry(*supporter).or_default().insert(brick_index);
            }
            self.supported_by.insert(brick_index, supporters);

            // Move the brick down so it's bottom is at highest_z + 1 (i.e. settle it)
            let delta = brick.bottom() - (highest_z + 1);
            let brick = &mut self.bricks[brick_index];
            brick.drop(delta);

            // println!("Brick {} settled at z={}-{}", brick.label, brick.bottom(), brick.top());
        }
    }

    // A brick is "disintegratable" if it is not the sole supporter of any other brick
    fn count_disintegratable_bricks(&self) -> usize {
        let mut candidates: HashSet<usize> = HashSet::from_iter(0..self.bricks.len());
        for supporters_indices in self.supported_by.values() {
            if supporters_indices.len() == 1 {
                // There's only one brick in the supporters_indices, so the brick in that set
                // is not disintegratable (doing so would cause the brick it supports to fall)
                candidates.remove(supporters_indices.iter().next().unwrap());
            }
        }
        candidates.len()
    }

    fn sum_num_transitively_singly_supported_bricks(&self, brick_index: usize) -> usize {
        let mut falling = HashSet::new();
        falling.insert(brick_index);
        let mut seen = HashSet::new();
        let mut to_visit = vec![brick_index];
        while let Some(brick_index) = to_visit.pop() {
            if seen.insert(brick_index) {
                if let Some(supporters) = self.supports.get(&brick_index) {
                    for supporter in supporters {
                        if let Some(supported_by) = self.supported_by.get(supporter) {
                            if supported_by.iter().all(|&i| falling.contains(&i)) {
                                to_visit.push(*supporter);
                                falling.insert(*supporter);
                            }
                        }
                    }
                }
            }
        }
        seen.len() - 1 // Don't count the brick itself
    }

    fn sum_num_supported_bricks_for_each_brick(&self) -> usize {
        (0..self.bricks.len())
            .map(|brick_index| self.sum_num_transitively_singly_supported_bricks(brick_index))
            .sum()
    }
}

// Converts to a number to a base-26 string, but using A-Z instead of any numeric characters
fn num_to_alpha(num: usize) -> String {
    if num == 0 {
        return "A".into();
    }
    let mut num = num;
    let mut result = String::new();
    while num > 0 {
        let digit = num % 26;
        result.push((b'A' + digit as u8) as char);
        num /= 26;
    }
    result.chars().rev().collect()
}

fn main() {
    let input = include_str!("../../input/day22");
    let mut snapshot = Snapshot::parse(input);
    snapshot.settle();
    println!("Part 1: {}", snapshot.count_disintegratable_bricks());
    println!("Part 2: {}", snapshot.sum_num_supported_bricks_for_each_brick());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_count_disintegratable_bricks() {
        let mut snapshot = Snapshot::parse(EXAMPLE);
        snapshot.settle();
        assert_eq!(snapshot.count_disintegratable_bricks(), 5);
    }

    #[test]
    fn test_sum_num_supported_bricks_for_each_brick() {
        let mut snapshot = Snapshot::parse(EXAMPLE);
        snapshot.settle();
        assert_eq!(snapshot.sum_num_supported_bricks_for_each_brick(), 7);
    }
}