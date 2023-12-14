use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Hash)]
enum Cell {
    Round,
    Square,
    Empty,
}
#[derive(PartialEq, Eq, Clone, Hash)]
struct Platform {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}
impl Platform {
    fn parse(input: &str) -> Self {
        let cells = input.lines().map(|line| {
            line.bytes().map(|b| {
                match b {
                    b'.' => Cell::Empty,
                    b'#' => Cell::Square,
                    b'O' => Cell::Round,
                    c => panic!("Invalid input {}", c),
                }
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        let width = cells[0].len();
        let height = cells.len();
        Platform {
            cells,
            width,
            height,
        }
    }

    fn tilt_north(&mut self) {
        for col in 0..self.width {
            let mut top = 0;
            while self.cells[top][col] != Cell::Empty {
                top += 1;
            }
            let mut bottom = top + 1;
            while bottom < self.height {
                match self.cells[bottom][col] {
                    Cell::Empty => bottom += 1,
                    Cell::Square => {
                        top = bottom + 1;
                        while top < self.height && self.cells[top][col] != Cell::Empty {
                            top += 1;
                        }
                        if top >= self.height {
                            break;
                        }
                        bottom = top + 1;
                    }
                    Cell::Round => {
                        self.cells[top][col] = Cell::Round;
                        self.cells[bottom][col] = Cell::Empty;
                        top += 1;
                        while top < self.height && self.cells[top][col] != Cell::Empty {
                            top += 1;
                        }
                        if top >= self.height {
                            break;
                        }
                        bottom += 1;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for col in 0..self.width {
            let mut bottom = self.height - 1;
            while self.cells[bottom][col] != Cell::Empty {
                bottom -= 1;
            }
            let mut top = bottom - 1;
            while top > 0 {
                match self.cells[top][col] {
                    Cell::Empty => top -= 1,
                    Cell::Square => {
                        if top == 0 {
                            break;
                        }
                        bottom = top - 1;
                        while bottom > 0 && self.cells[bottom][col] != Cell::Empty {
                            bottom -= 1;
                        }
                        if bottom == 0 {
                            break;
                        }
                        top = bottom - 1;
                    }
                    Cell::Round => {
                        self.cells[top][col] = Cell::Empty;
                        self.cells[bottom][col] = Cell::Round;
                        if bottom == 0 {
                            break;
                        }
                        bottom -= 1;
                        while bottom > 0 && self.cells[bottom][col] != Cell::Empty {
                            bottom -= 1;
                        }
                        if self.cells[bottom][col] != Cell::Empty {
                            break;
                        }
                        top -= 1;
                    }
                }
            }
            if top == 0 && self.cells[top][col] == Cell::Round {
                self.cells[top][col] = Cell::Empty;
                self.cells[bottom][col] = Cell::Round;
            }
        }
    }

    fn tilt_west(&mut self) {
        for row in 0..self.height {
            let mut left = 0;
            while self.cells[row][left] != Cell::Empty {
                left += 1;
            }
            let mut right = left + 1;
            while right < self.width {
                match self.cells[row][right] {
                    Cell::Empty => right += 1,
                    Cell::Square => {
                        left = right + 1;
                        while left < self.width && self.cells[row][left] != Cell::Empty {
                            left += 1;
                        }
                        if left >= self.width {
                            break;
                        }
                        right = left + 1;
                    }
                    Cell::Round => {
                        self.cells[row][left] = Cell::Round;
                        self.cells[row][right] = Cell::Empty;
                        left += 1;
                        while left < self.width && self.cells[row][left] != Cell::Empty {
                            left += 1;
                        }
                        if left >= self.width {
                            break;
                        }
                        right += 1;
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for row in 0..self.height {
            let mut right = self.width - 1;
            while self.cells[row][right] != Cell::Empty {
                right -= 1;
            }
            let mut left = right - 1;
            while left > 0 {
                match self.cells[row][left] {
                    Cell::Empty => left -= 1,
                    Cell::Square => {
                        if left == 0 {
                            break;
                        }
                        right = left - 1;
                        while right > 0 && self.cells[row][right] != Cell::Empty {
                            right -= 1;
                        }
                        if right == 0 {
                            break;
                        }
                        left = right - 1;
                    }
                    Cell::Round => {
                        self.cells[row][left] = Cell::Empty;
                        self.cells[row][right] = Cell::Round;
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                        while right > 0 && self.cells[row][right] != Cell::Empty {
                            right -= 1;
                        }
                        if self.cells[row][right] != Cell::Empty {
                            break;
                        }
                        left -= 1;
                    }
                }
            }
            if left == 0 && self.cells[row][left] == Cell::Round {
                self.cells[row][left] = Cell::Empty;
                self.cells[row][right] = Cell::Round;
            }
        }
    }

    fn calc_north_load(&self) -> usize {
        // Every Round in row 0 is a load of self.height, in row 1 is a load of self.height - 1, etc.
        self.cells.iter().enumerate().map(|(row_index, row)| {
            row.iter().filter(|cell| **cell == Cell::Round).count() * (self.height - row_index)
        }).sum()
    }

    // fn debug_print(&self) {
    //     for row in &self.cells {
    //         for cell in row {
    //             match cell {
    //                 Cell::Empty => print!("."),
    //                 Cell::Square => print!("#"),
    //                 Cell::Round => print!("O"),
    //             }
    //         }
    //         println!();
    //     }
    // }
}

fn main() {
    let input = include_str!("../../input/day14");
    let mut platform = Platform::parse(input);
    platform.tilt_north();
    println!("Part 1: {}", platform.calc_north_load());

    let mut platform = Platform::parse(input);
    let mut loop_info = None;
    let mut cache = HashMap::new();
    for i in 0..1000000000 {
        platform.tilt_north();
        platform.tilt_west();
        platform.tilt_south();
        platform.tilt_east();
        if let Some(loop_start) = cache.insert(platform.clone(), i + 1) {
            loop_info = Some((i + 1 - loop_start, i + 1));
            break;
        }
    }
    let (loop_length, cycles_completed) = loop_info.unwrap();
    let cycles_remaining = 1000000000 - cycles_completed;
    let remainder_after_loop = cycles_remaining % loop_length;
    for _ in 0..remainder_after_loop {
        platform.tilt_north();
        platform.tilt_west();
        platform.tilt_south();
        platform.tilt_east();
    }
    println!("Part 2: {}", platform.calc_north_load());
}