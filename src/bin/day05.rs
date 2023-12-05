#![feature(iter_array_chunks)]

use std::collections::HashMap;

struct MapRange {
    source_start: usize,
    dest_start: usize,
    length: usize,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Debug)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

use Category::*;
const CATEGORY_PAIRS: [(Category, Category); 7] = [
    (Seed, Soil),
    (Soil, Fertilizer),
    (Fertilizer, Water),
    (Water, Light),
    (Light, Temperature),
    (Temperature, Humidity),
    (Humidity, Location),
];

fn parse(input: &str) -> (Vec<usize>, HashMap<Category, Vec<MapRange>>) {
    let mut lines = input.lines();

    let start_seeds = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // Consume empty line, and first section title
    lines.next();
    lines.next();

    let mut mappings_by_category = HashMap::new();

    for (source, _dest) in &CATEGORY_PAIRS {
        let mut category_maps = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                // Consume next section title
                lines.next();
                break;
            }
            let mut nums = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap());
            let dest_start = nums.next().unwrap();
            let source_start = nums.next().unwrap();
            let length = nums.next().unwrap();
            category_maps.push(MapRange {
                source_start,
                dest_start,
                length,
            });
        }
        mappings_by_category.insert(source.clone(), category_maps);
    }

    (start_seeds, mappings_by_category)
}

fn part1(input: &str) -> usize {
    let (start_seeds, mappings_by_category) = parse(input);

    start_seeds
        .iter()
        .map(|seed| {
            let mut source_id = *seed;
            for (source, _dest) in &CATEGORY_PAIRS {
                let map_ranges = mappings_by_category.get(source).unwrap();
                let dest_id = map_ranges
                    .iter()
                    .find(|mr| {
                        mr.source_start <= source_id && source_id < mr.source_start + mr.length
                    })
                    .map(|mr| mr.dest_start + source_id - mr.source_start)
                    .unwrap_or(source_id);
                source_id = dest_id;
            }
            source_id
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let (start_seeds, mappings_by_category) = parse(input);

    let start_seed_ranges = start_seeds.into_iter().array_chunks::<2>().collect::<Vec<_>>();

    start_seed_ranges.into_iter()
        .flat_map(|[seed_range_start, seed_range_length]| {
            (seed_range_start..seed_range_start + seed_range_length)
                .map(|seed| {
                    let mut source_id = seed;
                    for (source, _dest) in &CATEGORY_PAIRS {
                        let map_ranges = mappings_by_category.get(source).unwrap();
                        let dest_id = map_ranges
                            .iter()
                            .find(|mr| {
                                mr.source_start <= source_id && source_id < mr.source_start + mr.length
                            })
                            .map(|mr| mr.dest_start + source_id - mr.source_start)
                            .unwrap_or(source_id);
                        source_id = dest_id;
                    }
                    source_id
                })
        })
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../../input/day05");

    let nearest_location = part1(input);
    println!("Part 1: {}", nearest_location);

    let nearest_location_2 = part2(input);
    println!("Part 2: {}", nearest_location_2);
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn example_part1() {
        assert_eq!(super::part1(EXAMPLE), 35);
    }

    #[test]
    fn example_part2() {
        assert_eq!(super::part2(EXAMPLE), 46);
    }
}
