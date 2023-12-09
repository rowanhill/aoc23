#![feature(iter_map_windows)]

fn parse(input: &str) -> Vec<Vec<isize>> {
    input.lines().map(|line| {
        line.split_whitespace().map(|num| num.parse().unwrap()).collect()
    }).collect()
}

fn derivatives(nums: &[isize]) -> Vec<isize> {
    nums.iter().map_windows(|[&a, &b]| b - a).collect()
}

fn predict_next(nums: &[isize]) -> isize {
    if nums.iter().all(|&n| n == 0) {
        0
    } else {
        nums.last().unwrap() + predict_next(&derivatives(nums))
    }
}

fn predict_prev(nums: &[isize]) -> isize {
    if nums.iter().all(|&n| n == 0) {
        0
    } else {
        nums.first().unwrap() - predict_prev(&derivatives(nums))
    }
}

fn part1(input: &str) -> isize {
    let nums = parse(input);
    nums.iter().map(|nums| predict_next(nums)).sum()
}

fn part2(input: &str) -> isize {
    let nums = parse(input);
    nums.iter().map(|nums| predict_prev(nums)).sum()
}

fn main() {
    let input = include_str!("../../input/day09");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 2);
    }
}