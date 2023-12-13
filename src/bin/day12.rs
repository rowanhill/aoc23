use std::collections::HashMap;

struct SpringState<'a> {
    chars: &'a [char],
    damanged_lengths: &'a [u8],
}

impl<'a> SpringState<'a> {
    fn new(chars: &'a [char], damanged_lengths: &'a [u8]) -> Self {
        Self {
            chars,
            damanged_lengths,
        }
    }

    fn count_possible_arrangements(&self, cache: &mut HashMap<(usize, usize), usize>) -> usize {
        let key = (self.chars.len(), self.damanged_lengths.len());
        if let Some(&x) = cache.get(&key) {
            return x;
        }

        if self.is_complete() {
            return 1;
        }

        let mut sum = 0;
        if let Some(next_state) = self.consume_operational() {
            sum += next_state.count_possible_arrangements(cache);
        }
        if let Some(next_state) = self.consume_damaged() {
            sum += next_state.count_possible_arrangements(cache);
        }

        cache.insert(key, sum);
        sum
    }

    fn is_complete(&self) -> bool {
        self.damanged_lengths.is_empty() && self.chars.iter().all(|&c| c == '.')
    }

    fn consume_operational(&self) -> Option<Self> {
        if self.chars.is_empty() || !(self.chars[0] == '.' || self.chars[0] == '?') {
            return None;
        }
        Some(Self {
            chars: &self.chars[1..],
            damanged_lengths: self.damanged_lengths,
        })
    }

    fn consume_damaged(&self) -> Option<Self> {
        if self.chars.is_empty() || !(self.chars[0] == '#' || self.chars[0] == '?') || self.damanged_lengths.is_empty() {
            return None;
        }

        let next_damaged_segments_count = self.damanged_lengths[0];

        if self.chars.len() < next_damaged_segments_count as usize {
            return None;
        }
        if self.chars[0..next_damaged_segments_count as usize].iter().any(|&c| !(c == '#' || c == '?')) {
            return None;
        }

        // Character after damaged segment must be operational (. or ?), or end of string
        match self.chars.get(next_damaged_segments_count as usize) {
            Some('#') => None,
            None => Some(Self {
                chars: &self.chars[(next_damaged_segments_count as usize)..],
                damanged_lengths: &self.damanged_lengths[1..],
            }),
            Some('.') | Some('?') => Some(Self {
                chars: &self.chars[(next_damaged_segments_count as usize + 1)..],
                damanged_lengths: &self.damanged_lengths[1..],
            }),
            Some(c) => panic!("Unexpected character {}", c),
        }
    }
}

fn parse_line(line: &str) -> (Vec<char>, Vec<u8>) {
    let (left, right) = line.split_once(' ').unwrap();
    (left.chars().collect::<Vec<_>>(), right.split(',').map(|x| x.parse().unwrap()).collect::<Vec<_>>())
}

fn count_possible_arrangements(line: &str) -> usize {
    let (chars, damaged_lengths) = parse_line(line);
    let state = SpringState::new(&chars, &damaged_lengths);
    let mut cache = HashMap::new();
    state.count_possible_arrangements(&mut cache)
}

fn count_possible_arrangements_unfolded(line: &str) -> usize {
    let (chars, damaged_lengths) = parse_line(line);
    
    // Repeat the full sequence of chars 5 times, separating each repetition with a '?'
    let mut unfolded_chars = Vec::new();
    for i in 0..5 {
        unfolded_chars.extend_from_slice(&chars);
        if i < 4 {
            unfolded_chars.push('?');
        }
    }

    // Repeat the full sequence of damaged lengths 5 times
    let mut unfolded_damaged_lengths = Vec::new();
    for _ in 0..5 {
        unfolded_damaged_lengths.extend_from_slice(&damaged_lengths);
    }

    let state = SpringState::new(&unfolded_chars, &unfolded_damaged_lengths);
    let mut cache = HashMap::new();
    state.count_possible_arrangements(&mut cache)
}

fn main() {
    let input = include_str!("../../input/day12");

    let sum_of_arrangements = input.lines().map(count_possible_arrangements).sum::<usize>();
    println!("Part 1: {}", sum_of_arrangements);

    let sum_of_unfolded_arrangements = input.lines().map(count_possible_arrangements_unfolded).sum::<usize>();
    println!("Part 2: {}", sum_of_unfolded_arrangements);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(count_possible_arrangements("???.### 1,1,3"), 1);
        assert_eq!(count_possible_arrangements(".??..??...?##. 1,1,3"), 4);
        assert_eq!(count_possible_arrangements("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(count_possible_arrangements("????.#...#... 4,1,1"), 1);
        assert_eq!(count_possible_arrangements("????.######..#####. 1,6,5"), 4);
        assert_eq!(count_possible_arrangements("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(count_possible_arrangements_unfolded("???.### 1,1,3"), 1);
        // assert_eq!(count_possible_arrangements_unfolded(".??..??...?##. 1,1,3"), 16384);
        // assert_eq!(count_possible_arrangements_unfolded("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        // assert_eq!(count_possible_arrangements_unfolded("????.#...#... 4,1,1"), 16);
        // assert_eq!(count_possible_arrangements_unfolded("????.######..#####. 1,6,5"), 2500);
        assert_eq!(count_possible_arrangements_unfolded("?###???????? 3,2,1"), 506250);
    }
}