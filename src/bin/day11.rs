#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
struct Coord {
    x: usize,
    y: usize,
}
struct Image {
    galaxies: Vec<Coord>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

fn parse(input: &str) -> Image {
    let mut width = 0;
    let mut height = 0;
    let mut galaxies = Vec::new();
    for (col_index, line) in input.lines().enumerate() {
        for (row_index, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Coord { x: row_index, y: col_index });
            }
            width = row_index + 1;
        }
        height = col_index + 1;
    }

    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    for x in 0..width {
        if !galaxies.iter().any(|c| c.x == x) {
            empty_cols.push(x);
        }
    }
    for y in 0..height {
        if !galaxies.iter().any(|c| c.y == y) {
            empty_rows.push(y);
        }
    }
    
    Image {
        galaxies,
        empty_rows,
        empty_cols,
    }
}

fn calc_sum_of_dists(image: &Image, empty_count_as: usize) -> usize {
    let mut dists_sum = 0;
    for i in 0..image.galaxies.len() {
        for j in (i+1)..image.galaxies.len() {
            let source = &image.galaxies[i];
            let dest = &image.galaxies[j];

            let left = source.x.min(dest.x);
            let right = source.x.max(dest.x);
            let top = source.y.min(dest.y);
            let bottom = source.y.max(dest.y);

            let num_empty_cols_spanned = image.empty_cols.iter()
                .filter(|x| **x > left && **x <= right)
                .count();
            let num_empty_rows_spanned = image.empty_rows.iter()
                .filter(|y| **y > top && **y <= bottom)
                .count();
            
            let dist = (right - left) + (bottom - top) + num_empty_cols_spanned * (empty_count_as - 1) + num_empty_rows_spanned * (empty_count_as - 1);
            dists_sum += dist;
        }
    }
    dists_sum
}

fn main() {
    let input = include_str!("../../input/day11");
    let image = parse(input);

    println!("Part 1: {}", calc_sum_of_dists(&image, 2));
    println!("Part 2: {}", calc_sum_of_dists(&image, 1_000_000));
}