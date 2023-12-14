use std::{collections::HashSet, cmp::Ordering};

struct Pattern {
    grid: Vec<Vec<bool>>, // true => # (i.e. rock), false => . (i.e. ash)
    width: usize,
    height: usize,
}

impl Pattern {
    // Detects whether there is a reflection in the grid either horizontally or vertically.
    // For vertical reflection, the "summary" is the number of columns to the left of the line of reflection
    // For horizontal reflection, the "summary" is the number of rows above the line of reflection multiplied by 100
    fn reflection_summary(&self) -> usize {
        // Check for vertical reflection - start with a hashset of all possible columns, and remove entries as we
        // prove they don't reflect
        let mut possible_columns = (1..self.width).collect::<HashSet<_>>();
        for row in 0..self.height {
            possible_columns.retain(|col| {
                let max_offset = (col - 1).min(self.width - col - 1);
                (0..=max_offset).all(|offset| self.grid[row][col - offset - 1] == self.grid[row][col + offset])
            });
        }
        match possible_columns.len().cmp(&1) {
            Ordering::Equal => return *possible_columns.iter().next().unwrap(),
            Ordering::Greater => panic!("More than one possible column for vertical reflection"),
            _ => {},
        };

        // Repeat the above, but checking for horizontal lines of reflection
        let mut possible_rows = (1..self.height).collect::<HashSet<_>>();
        for col in 0..self.width {
            possible_rows.retain(|row| {
                let max_offset = (row - 1).min(self.height - row - 1);
                (0..=max_offset).all(|offset| self.grid[row - offset - 1][col] == self.grid[row + offset][col])
            });
        }
        match possible_rows.len().cmp(&1) {
            Ordering::Equal => return *possible_rows.iter().next().unwrap() * 100,
            Ordering::Greater => panic!("More than one possible row for horizontal reflection"),
            _ => {},
        };
        
        panic!("No reflection found");
    }

    // Behaves as reflection_summary, but where one single bool is smudged (i.e. flipped from true to false or vice versa)
    fn reflection_summary_with_one_smudge(&self) -> usize {
        for col in 1..self.width {
            if self.has_smudge_vertical(col) {
                return col;
            }
        }

        for row in 1..self.height {
            if self.has_smudge_horizontal(row) {
                return row * 100;
            }
        }

        panic!("No smudge found");
    }

    fn has_smudge_vertical(&self, cols_to_left: usize) -> bool {
        let mut has_allocated_one_smudge = false;
        for row in 0..self.height {
            let max_offset = (cols_to_left - 1).min(self.width - cols_to_left - 1);
            for offset in 0..=max_offset {
                if self.grid[row][cols_to_left - 1 - offset] != self.grid[row][cols_to_left + offset] {
                    if has_allocated_one_smudge {
                        return false;
                    } else {
                        has_allocated_one_smudge = true;
                    }
                }
            }
        }
        has_allocated_one_smudge
    }

    fn has_smudge_horizontal(&self, rows_above: usize) -> bool {
        let mut has_allocated_one_smudge = false;
        for col in 0..self.width {
            let max_offset = (rows_above - 1).min(self.height - rows_above - 1);
            for offset in 0..=max_offset {
                if self.grid[rows_above - 1 - offset][col] != self.grid[rows_above + offset][col] {
                    if has_allocated_one_smudge {
                        return false;
                    } else {
                        has_allocated_one_smudge = true;
                    }
                }
            }
        }
        has_allocated_one_smudge
    }
}

fn parse(input: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut width = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(Pattern {
                height: grid.len(),
                grid,
                width,
            });
            grid = Vec::new();
            continue;
        }
        let row = line.bytes().map(|b| b == b'#').collect::<Vec<_>>();
        width = row.len();
        grid.push(row);
    }
    patterns.push(Pattern {
        height: grid.len(),
        grid,
        width,
    });
    patterns

}

fn main() {
    let input = include_str!("../../input/day13");
    let patterns = parse(input);
    println!("Part 1: {}", patterns.iter().map(|p| p.reflection_summary()).sum::<usize>());
    println!("Part 2: {}", patterns.iter().map(|p| p.reflection_summary_with_one_smudge()).sum::<usize>());
}