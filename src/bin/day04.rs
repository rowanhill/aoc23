use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/day04");

    let mut sum_of_points = 0_usize;
    let mut num_of_copies = input.lines().map(|_| 1_usize).collect::<Vec<_>>();
    for (index, line) in input.lines().enumerate() {
        let (_, all_nums) = line.split_once(": ").unwrap();
        let (winning, actual) = all_nums.split_once(" | ").unwrap();
        let winning = winning.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let actual = actual.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect::<HashSet<_>>();

        let count_of_winning_nums = winning.iter().filter(|n| actual.contains(n)).count();

        let points_for_card = if count_of_winning_nums > 0 { 2_usize.pow(count_of_winning_nums as u32 - 1) } else { 0 };
        sum_of_points += points_for_card;

        // For the N next cards, where N is the count of winning numbers, increase the number of cards by the
        // number of copies of the current card
        for i in index + 1..(index + 1 + count_of_winning_nums).min(num_of_copies.len()) {
            num_of_copies[i] += num_of_copies[index];
        }
    }

    println!("Part 1: {}", sum_of_points);

    let total_cards: usize = num_of_copies.iter().sum();
    println!("Part 2: {}", total_cards);
}