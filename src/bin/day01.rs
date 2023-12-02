fn main() {
  let input = include_str!("../../input/day01");
  let sum_of_digits = input.lines()
    .map(|line| find_first_digit(line) * 10 + find_last_digit(line))
    .sum::<u32>();
  println!("Part 1: {}", sum_of_digits);
  let sum_of_digits_or_numbers = input.lines()
    .map(|line| find_first_digit_or_number(line) * 10 + find_last_digit_or_number(line))
    .sum::<usize>();
  println!("Part 2: {}", sum_of_digits_or_numbers);
}

fn find_first_digit(s: &str) -> u32 {
  s.chars().find(|c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap()
}

fn find_last_digit(s: &str) -> u32 {
  s.chars().rev().find(|c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap()
}

const NEEDLES: [&str; 20] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
                             "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_first_digit_or_number(s: &str) -> usize {
  NEEDLES.iter()
    .map(|needle| s.find(needle))
    .enumerate()
    .filter(|(_, idx)| idx.is_some())
    .min_by_key(|(_, idx)| *idx)
    .map(|(needle_idx, _)| needle_idx % 10)
    .unwrap()
}

fn find_last_digit_or_number(s: &str) -> usize {
  NEEDLES.iter()
    .map(|needle| s.rfind(needle))
    .enumerate()
    .filter(|(_, idx)| idx.is_some())
    .max_by_key(|(_, idx)| *idx)
    .map(|(needle_idx, _)| needle_idx % 10)
    .unwrap()
}