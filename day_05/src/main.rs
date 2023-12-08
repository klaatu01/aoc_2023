fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // benchmark
    let now = std::time::Instant::now();
    println!("Part 01: {}", part_01(&input));
    println!("Time: {}µs", now.elapsed().as_micros());

    let now = std::time::Instant::now();
    println!("Part 02: {}", part_02(&input));
    println!("Time: {}µs", now.elapsed().as_micros());
}

struct MapRanges(Vec<MapRange>);

struct MapRange {
    source: usize,
    destination: usize,
    offset: usize,
}

fn remap(lo: usize, hi: usize, m: &MapRanges) -> Vec<(usize, usize)> {
    let ans = m
        .inner()
        .iter()
        .filter_map(
            |MapRange {
                 destination,
                 source,
                 offset,
             }| {
                let end = source + offset - 1;
                let d = *destination as isize - *source as isize; // Shift amount

                if !(end < lo || *source > hi) {
                    Some((usize::max(*source, lo), usize::min(end, hi), d))
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();

    let mut result = ans
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, &(l, r, d))| {
            acc.push((l.wrapping_add(d as usize), r.wrapping_add(d as usize)));

            if i < ans.len() - 1 && ans[i + 1].0 > r + 1 {
                acc.push((r + 1, ans[i + 1].0 - 1));
            }

            acc
        });

    if ans.is_empty() {
        return vec![(lo, hi)];
    }

    if ans[0].0 > lo {
        result.insert(0, (lo, ans[0].0 - 1));
    }
    if ans.last().unwrap().1 < hi {
        result.push((ans.last().unwrap().1 + 1, hi));
    }

    result
}

impl MapRange {
    pub fn new(source: usize, destination: usize, offset: usize) -> Self {
        Self {
            source,
            destination,
            offset,
        }
    }

    pub fn map(&self, input: usize) -> Option<usize> {
        if self.source <= input && input < self.source + self.offset {
            Some(input - self.source + self.destination)
        } else {
            None
        }
    }
}

impl MapRanges {
    fn map(&self, input: usize) -> usize {
        self.0
            .iter()
            .find_map(|map| map.map(input))
            .unwrap_or(input)
    }

    fn inner(&self) -> &Vec<MapRange> {
        &self.0
    }
}

fn parse_seeds(input: &str) -> Vec<usize> {
    input
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_ranges(input: Vec<&str>) -> Vec<MapRange> {
    input
        .iter()
        .skip(1)
        .flat_map(|line| match line.len() {
            0 => None,
            _ => {
                let mut parts = line.split(" ");
                let destination = parts.next().unwrap().parse().unwrap();
                let source = parts.next().unwrap().parse().unwrap();
                let offset = parts.next().unwrap().parse().unwrap();
                Some(MapRange::new(source, destination, offset))
            }
        })
        .collect()
}

fn parse(sections: Vec<&str>) -> (Vec<usize>, Vec<MapRanges>) {
    let seeds: Vec<usize> = sections
        .first()
        .map(|section| parse_seeds(section))
        .unwrap();

    let maps: Vec<MapRanges> = sections
        .iter()
        .skip(1)
        .map(|section| {
            let lines = section.split("\n").collect::<Vec<_>>();
            let mut ranges = parse_ranges(lines);
            ranges.sort_by(|a, b| a.source.cmp(&b.source));
            MapRanges(ranges)
        })
        .collect();

    (seeds, maps)
}

fn part_01(input: &str) -> usize {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let (seeds, maps) = parse(sections);
    seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |acc, map| map.map(acc)))
        .min()
        .unwrap()
}

fn part_02(input: &str) -> usize {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let (seeds, maps) = parse(sections);

    let range_seeds: Vec<(usize, usize)> = seeds.chunks(2).map(|c| (c[0], c[0] + c[1])).collect();

    range_seeds.iter().fold(usize::MAX, |acc, &(start, r)| {
        let cur_intervals = maps.iter().fold(vec![(start, r)], |cur_intervals, map| {
            cur_intervals
                .iter()
                .flat_map(|&(lo, hi)| remap(lo, hi, map))
                .collect()
        });

        cur_intervals
            .iter()
            .fold(acc, |min_ans, &(lo, _)| usize::min(min_ans, lo))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_source() {
        let map = MapRange {
            destination: 50,
            source: 98,
            offset: 2,
        };

        assert_eq!(map.map(98), Some(50));
        assert_eq!(map.map(99), Some(51));
        assert_eq!(map.map(100), None);
    }

    #[test]
    fn test_map_destination() {
        let maps = MapRanges(vec![
            MapRange {
                destination: 50,
                source: 98,
                offset: 2,
            },
            MapRange {
                destination: 52,
                source: 50,
                offset: 48,
            },
        ]);

        assert_eq!(maps.map(0), 0);
        assert_eq!(maps.map(1), 1);
        assert_eq!(maps.map(48), 48);
        assert_eq!(maps.map(49), 49);
        assert_eq!(maps.map(50), 52);
        assert_eq!(maps.map(51), 53);
        assert_eq!(maps.map(96), 98);
        assert_eq!(maps.map(97), 99);
        assert_eq!(maps.map(98), 50);
        assert_eq!(maps.map(99), 51);
    }

    #[test]
    fn test_part_01() {
        let input = "seeds: 79 14 55 13

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
        assert_eq!(part_01(input), 35);
    }

    #[test]
    fn test_part_02() {
        let input = "seeds: 79 14 55 13

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
        assert_eq!(part_02(input), 46);
    }
}
