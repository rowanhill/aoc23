#![feature(iter_array_chunks)]

use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}
struct MultiRangeMap {
    source: Vec<Range>,
    dest: Vec<Range>,
}
struct MultiRange {
    ranges: Vec<Range>,
}

impl MultiRangeMap {
    fn map(&self, input: &MultiRange) -> MultiRange {
        let mut output = MultiRange { ranges: Vec::new() };

        let mut queue = VecDeque::new();
        queue.extend(input.ranges.clone());

        for (source_range, dest_range) in self.source.iter().zip(&self.dest) {
            let mut remainders = Vec::new();
            while let Some(input_range) = queue.pop_front() {
                let (left, intersection, right) = input_range.intersect(source_range);
                remainders.extend(left);
                remainders.extend(right);
                let mapped_intersection = intersection.map(|r| Range {
                    start: dest_range.start + r.start - source_range.start,
                    end: dest_range.start + r.end - source_range.start,
                });
                if let Some(mapped) = mapped_intersection {
                    output.merge_range(&mapped);
                }
            }
            queue.extend(remainders);
        }
        
        for range in queue {
            output.merge_range(&range);
        }

        output
    }
}

impl MultiRange {
    fn merge_range(&mut self, range: &Range) {
        let mut range = range.clone();
        let mut new_ranges = Vec::new();
        for r in &self.ranges {
            if r.overlaps_or_abuts(&range) {
                range = Range {
                    start: range.start.min(r.start),
                    end: range.end.max(r.end),
                };
            } else {
                new_ranges.push(r.clone());
            }
        }
        new_ranges.push(range);
        new_ranges.sort_by_key(|r| r.start);
        self.ranges = new_ranges;
    }
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    fn overlaps_or_abuts(&self, other: &Range) -> bool {
        self.start <= other.end + 1 && other.start <= self.end + 1
    }

    fn intersect(&self, other: &Range) -> (Option<Range>, Option<Range>, Option<Range>) {
        if self.overlaps(other) {
            (
                if self.start < other.start {
                    Some(Range {
                        start: self.start,
                        end: other.start - 1,
                    })
                } else {
                    None
                },
                Some(Range {
                    start: self.start.max(other.start),
                    end: self.end.min(other.end),
                }),
                if self.end > other.end {
                    Some(Range {
                        start: other.end + 1,
                        end: self.end,
                    })
                } else {
                    None
                },
            )
        } else if self.start < other.start {
            (Some(self.clone()), None, None)
        } else {
            (None, None, Some(self.clone()))
        }
    }
}

fn parse(input: &str) -> (Vec<usize>, Vec<MultiRangeMap>) {
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

    let mut multi_range_maps = Vec::new();

    let mut sources = Vec::new();
    let mut dests = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            // We're at the end of a section, so create a MultiRangeMap from the RangeMaps
            multi_range_maps.push(MultiRangeMap {
                source: sources,
                dest: dests,
            });
            sources = Vec::new();
            dests = Vec::new();

            // Consume next section title
            if lines.next().is_none() {
                break;
            }
            continue;
        }

        let mut nums = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap());
        let dest_start = nums.next().unwrap();
        let source_start = nums.next().unwrap();
        let length = nums.next().unwrap();

        sources.push(Range {
            start: source_start,
            end: source_start + length - 1,
        });
        dests.push(Range {
            start: dest_start,
            end: dest_start + length - 1,
        });
    }
    // We're at the end of the file, so create a MultiRangeMap from the final RangeMaps
    multi_range_maps.push(MultiRangeMap {
        source: sources,
        dest: dests,
    });

    (start_seeds, multi_range_maps)
}

fn part1(input: &str) -> usize {
    let (start_seeds, multi_range_maps) = parse(input);

    let start_seed_ranges = start_seeds.into_iter()
        .map(|start| Range { start, end: start + 1 })
        .collect::<Vec<_>>();

    let mut input_ranges = MultiRange { ranges: start_seed_ranges };

    for multi_range_map in multi_range_maps {
        input_ranges = multi_range_map.map(&input_ranges);
    }
    
    input_ranges.ranges.iter().map(|r| r.start).min().unwrap()
}

fn part2(input: &str) -> usize {
    let (start_seeds, multi_range_maps) = parse(input);

    let start_seed_ranges = start_seeds.into_iter()
        .array_chunks::<2>()
        .map(|[start, length]| Range { start, end: start + length - 1 })
        .collect::<Vec<_>>();

    let mut input_ranges = MultiRange { ranges: start_seed_ranges };

    for multi_range_map in multi_range_maps {
        input_ranges = multi_range_map.map(&input_ranges);
    }
    
    input_ranges.ranges.iter().map(|r| r.start).min().unwrap()
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
