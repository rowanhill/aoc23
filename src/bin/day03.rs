struct SchematicNumber {
    number: usize,
    row_index: usize,
    start_col_index: usize,
    end_col_index: usize,
}
struct Symbol {
    symbol: char,
    row_index: usize,
    col_index: usize,
}
struct Schematic {
    numbers: Vec<SchematicNumber>,
    symbols: Vec<Symbol>,
}

impl Schematic {
    fn parse(input: &str) -> Schematic {
        // Any adjacent ascii digits on the same row are considered part of the same number. Any other character than a . is considered a symbol.
        let mut current_number: Option<SchematicNumber> = None;
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();
        for (row_index, line) in input.lines().enumerate() {
            for (col_index, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    match current_number {
                        Some(ref mut number) => {
                            number.number = number.number * 10 + c.to_digit(10).unwrap() as usize;
                            number.end_col_index = col_index;
                        }
                        None => {
                            current_number = Some(SchematicNumber {
                                number: c.to_digit(10).unwrap() as usize,
                                row_index,
                                start_col_index: col_index,
                                end_col_index: col_index,
                            });
                        }
                    }
                } else {
                    if current_number.is_some() {
                        // We've reached the end of a number
                        let number = current_number.unwrap();
                        numbers.push(number);
                        current_number = None;
                    }
                    if c != '.' {
                        symbols.push(Symbol {
                            symbol: c,
                            row_index,
                            col_index,
                        });
                    }
                }
            }
            if current_number.is_some() {
                // We've reached the end of a number at the end of a line
                let number = current_number.unwrap();
                numbers.push(number);
                current_number = None;
            }
        }
        if current_number.is_some() {
            // We've reached the end of a number and the end of the input
            let number = current_number.unwrap();
            numbers.push(number);
        }
        Schematic { numbers, symbols }
    }

    fn sum_part_numbers(&self) -> usize {
        self.numbers.iter()
            .filter(|number| self.symbols.iter().any(|symbol| number.adjacent_to(symbol)))
            .map(|number| number.number)
            .sum()
    }

    fn sum_gear_ratios(&self) -> usize {
        self.symbols.iter()
            .filter(|symbol| symbol.symbol == '*')
            .map(|symbol| {
                self.numbers.iter()
                    .filter(|number| number.adjacent_to(symbol))
                    .map(|number| number.number)
                    .collect::<Vec<_>>()
                }
            )
            .filter(|numbers| numbers.len() == 2)
            .map(|numbers| numbers[0] * numbers[1])
            .sum()
    }
}

impl SchematicNumber {
    fn adjacent_to(&self, coord: &Symbol) -> bool {
        coord.row_index >= self.row_index.saturating_sub(1)
            && coord.row_index <= self.row_index.saturating_add(1)
            && coord.col_index >= self.start_col_index.saturating_sub(1)
            && coord.col_index <= self.end_col_index.saturating_add(1)
    }

}

fn main() {
    let input = include_str!("../../input/day03");
    let schematic = Schematic::parse(input);
    let part_numbers_sum = schematic.sum_part_numbers();
    println!("Part 1: {}", part_numbers_sum);
    let gear_ratios_sum = schematic.sum_gear_ratios();
    println!("Part 2: {}", gear_ratios_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let schematic = Schematic::parse(input);
        let part_numbers_sum = schematic.sum_part_numbers();
        assert_eq!(part_numbers_sum, 4361);
    }

    #[test]
    fn example_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let schematic = Schematic::parse(input);
        let gear_ratios_sum = schematic.sum_gear_ratios();
        assert_eq!(gear_ratios_sum, 467835);
    }
}