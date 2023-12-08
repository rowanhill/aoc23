use std::collections::HashMap;

fn parse(input: &str) -> (Vec<u8>, HashMap<String, [String; 2]>) {
    let mut lines = input.lines();

    let turns = lines.next()
        .unwrap()
        .bytes()
        .map(|b| match b {
            b'L' => 0_u8,
            b'R' => 1,
            _ => unreachable!("Invalid input"),
        })
        .collect::<Vec<_>>();
    lines.next();

    let mut map = HashMap::new();
    for line in lines {
        let from = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        map.insert(from, [left, right]);
    }

    (turns, map)
}

fn part1(input: &str) -> usize {
    let (turns, map) = parse(input);

    let mut current = "AAA".to_string();
    let mut count = 0;
    while current != "ZZZ" {
        let turn = turns[count % turns.len()];
        let next = &map[&current][turn as usize];
        current = next.to_string();
        count += 1;
    }
    count
}

// Let's just guess / hope that the answer is the lowest common multiple of the number of steps
// it takes to reach a Z node from each starting A node.
// This is true for the example input, and feels like the kind of optimisation AoC would include,
// but I can't see that it's necessarily true!
fn part2(input: &str) -> usize {
    let (turns, map) = parse(input);

    let start_locations = map.keys().filter(|k| k.ends_with('A')).collect::<Vec<_>>();

    let num_steps = start_locations.into_iter()
        .map(|start_location| {
            let mut current = start_location;
            let mut count = 0;
            while !current.ends_with('Z') {
                let turn = turns[count % turns.len()];
                let next = &map[current][turn as usize];
                current = next;
                count += 1;
            }
            count
        })
        .collect::<Vec<_>>();

    lowest_common_multiple(num_steps)
}

fn lowest_common_multiple(nums: Vec<usize>) -> usize {
    let mut lcm = 1;
    for num in nums {
        lcm = lcm * num / gcd(lcm, num);
    }
    lcm
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let input = include_str!("../../input/day08");
    let num_turns_p1 = part1(input);
    println!("Part 1: {}", num_turns_p1);

    let num_turns_p2 = part2(input);
    println!("Part 2: {}", num_turns_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_1), 6);
    }

    const EXAMPLE_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_2), 6);
    }
}