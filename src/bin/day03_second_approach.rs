use std::collections::HashMap;

fn main() {
    let input = include_bytes!("../../input/day03");

    let schematic = input.split(|b| *b == b'\n').collect::<Vec<_>>();

    let mut current_number = None;
    let mut numbers = Vec::new();
    let mut next_id = 0_usize;
    let mut asterisks = HashMap::new();
    let mut sum_part_numbers = 0;

    let mut x = 0_usize;
    let mut y = 0_usize;
    for &byte in input {
        if byte.is_ascii_digit() {
            let digit = byte - b'0';
            current_number = match current_number {
                Some((start, n)) => Some((start, n * 10 + digit as usize)),
                None => Some((x, digit as usize)),
            };
        } else if let Some((start, number)) = current_number {
            let mut added_to_part_numbers = false;
            for symbol_y in y.saturating_sub(1)..=y.saturating_add(1) {
                for symbol_x in start.saturating_sub(1)..x.saturating_add(1) {
                    let Some(&symbol) = schematic.get(symbol_y).and_then(|row| row.get(symbol_x)) else {
                        continue;
                    };
                    if symbol != b'.' && !symbol.is_ascii_digit() {
                        if !added_to_part_numbers {
                            sum_part_numbers += number;
                            added_to_part_numbers = true;
                        }
                        if symbol == b'*' {
                            asterisks.entry((symbol_x, symbol_y)).or_insert_with(Vec::new).push(next_id);
                        }
                    }
                }
            }

            numbers.push(number);
            next_id += 1;
            current_number = None;
        }
        if byte == b'\n' {
            y += 1;
            x = 0;
        } else {
            x += 1;
        }
    }

    let sum_gear_ratios: usize = asterisks.iter()
        .filter(|(_, ids)| ids.len() == 2)
        .map(|(_, ids)| ids.iter().map(|id| numbers[*id]).product::<usize>())
        .sum();

    println!("Part 1: {}", sum_part_numbers);
    println!("Part 2: {}", sum_gear_ratios);
}