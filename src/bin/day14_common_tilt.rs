use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
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

    fn get(&self, row: isize, col: isize) -> Option<Cell> {
        if row < 0 || col < 0 {
            return None;
        }
        let row = row as usize;
        let col = col as usize;
        self.cells.get(row).and_then(|row| row.get(col)).cloned()
    }

    fn tilt<F, FSet>(&mut self, get: F, mut set: FSet, delta: isize, start: isize)
        where F: Fn(&Self, isize) -> Option<Cell>,
        FSet: FnMut(&mut Self, usize, Cell)
    {
        use Cell::*;
        let mut low = start; // The platform is tilted towards this end
        let mut high = start + delta; // and away from this end

        loop {
            match (get(self, low), get(self, high)) {
                (Some(Empty), Some(Round)) => {
                    set(self, low as usize, Round);
                    set(self, high as usize, Empty);
                    low += delta;
                    high += delta;
                },
                (_, Some(Square)) => {
                    low = high + delta;
                    high = low + delta;
                },
                (_, Some(Empty)) => {
                    high += delta;
                },
                (Some(Round), _) | (Some(Square), _) => {
                    low += delta;
                    high = low + delta;
                },
                (_, None) => break,
                default => panic!("Unexpected state {:?}", default),
            }
        }
    }

    fn tilt_north(&mut self) {
        for col in 0..self.width {
            let get = |platform: &Self, row| platform.get(row, col as isize);
            let set = |platform: &mut Self, row: usize, cell: Cell| platform.cells[row][col] = cell;
            self.tilt(get, set, 1, 0);
        }
    }

    fn tilt_south(&mut self) {
        for col in 0..self.width {
            let get = |platform: &Self, row| platform.get(row, col as isize);
            let set = |platform: &mut Self, row: usize, cell: Cell| platform.cells[row][col] = cell;
            self.tilt(get, set, -1, self.height as isize - 1);
        }
    }

    fn tilt_west(&mut self) {
        for row in 0..self.height {
            let get = |platform: &Self, col| platform.get(row as isize, col);
            let set = |platform: &mut Self, col: usize, cell: Cell| platform.cells[row][col] = cell;
            self.tilt(get, set, 1, 0);
        }
    }

    fn tilt_east(&mut self) {
        for row in 0..self.height {
            let get = |platform: &Self, col| platform.get(row as isize, col);
            let set = |platform: &mut Self, col: usize, cell: Cell| platform.cells[row][col] = cell;
            self.tilt(get, set, -1, self.width as isize - 1);
        }
    }

    fn calc_north_load(&self) -> usize {
        // Every Round in row 0 is a load of self.height, in row 1 is a load of self.height - 1, etc.
        self.cells.iter().enumerate().map(|(row_index, row)| {
            row.iter().filter(|cell| **cell == Cell::Round).count() * (self.height - row_index)
        }).sum()
    }
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