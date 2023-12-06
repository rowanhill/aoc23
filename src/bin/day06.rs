fn parse_multi(input: &str) -> Vec<(usize, usize)> {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let distance = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    time.into_iter().zip(distance).collect()
}

fn parse_single(input: &str) -> (usize, usize) {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();

    (time, distance)
}

fn count_ways_to_beat_record(record: &(usize, usize)) -> usize {
    let &(time, distance) = record;
    let mut count = 0;
    for charge_time in 1..=time {
        let remaining_time = time - charge_time;
        let distance_in_remaining_time = charge_time * remaining_time;
        if distance_in_remaining_time > distance {
            count += 1;
        }
    }
    count
}

fn part1(input: &str) -> usize {
    let race_records = parse_multi(input);
    race_records
        .iter()
        .map(count_ways_to_beat_record)
        .product::<usize>()
}

fn part2(input: &str) -> usize {
    let single_race_record = parse_single(input);
    count_ways_to_beat_record(&single_race_record)
}

fn main() {
    const INPUT: &str = include_str!("../../input/day06");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 71503);
    }
}