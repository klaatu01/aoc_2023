use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref PART_REGEX: regex::Regex = regex::Regex::new(r"(\d)+").unwrap();
    static ref SYMBOL_REGEX: regex::Regex = regex::Regex::new(r"[^\w\d.]").unwrap();
}

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

#[derive(Debug, Clone)]
struct Part {
    width: usize,
    value: usize,
    anchor: (usize, usize),
}

impl Part {
    fn get_adjacent_symbol(
        &self,
        symbol_map: &HashMap<(usize, usize), Symbol>,
    ) -> Option<((usize, usize), Symbol)> {
        let (x, y) = self.anchor;
        let width = self.width;
        for y in (y.saturating_sub(1))..(y + 2) {
            for x in (x.saturating_sub(1))..(x + width + 1) {
                if let Some(symbol) = symbol_map.get(&(x, y)) {
                    return Some(((x, y), symbol.clone()));
                }
            }
        }
        None
    }
}

type Symbol = char;

fn part_01(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| format!(".{}.", line))
        .collect::<Vec<_>>();

    let symbol_map: HashMap<(usize, usize), Symbol> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| SYMBOL_REGEX.is_match(&c.to_string()))
                .map(move |(x, c)| ((x, y), c))
        })
        .collect();

    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            PART_REGEX.find_iter(line).map(move |m| {
                let value = m.as_str().parse::<usize>().unwrap();
                let width = m.as_str().len();
                let anchor = (m.start(), y);
                Part {
                    width,
                    value,
                    anchor,
                }
            })
        })
        .filter(|part| part.get_adjacent_symbol(&symbol_map).is_some())
        .map(|part| part.value)
        .sum()
}

fn part_02(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| format!(".{}.", line))
        .collect::<Vec<_>>();

    let symbol_map: HashMap<(usize, usize), Symbol> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| SYMBOL_REGEX.is_match(&c.to_string()))
                .map(move |(x, c)| ((x, y), c))
        })
        .collect();

    let parts = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            PART_REGEX.find_iter(line).map(move |m| {
                let value = m.as_str().parse::<usize>().unwrap();
                let width = m.as_str().len();
                let anchor = (m.start(), y);
                Part {
                    width,
                    value,
                    anchor,
                }
            })
        })
        .flat_map(|part| {
            part.get_adjacent_symbol(&symbol_map)
                .map(|(pos, _)| (pos, part))
        })
        .collect::<Vec<((usize, usize), Part)>>();

    let mut overlaps: HashMap<(usize, usize), Vec<Part>> = HashMap::new();
    for (pos, part) in parts {
        overlaps
            .entry(pos)
            .or_insert_with(|| Vec::new())
            .push(part.clone());
    }

    overlaps
        .iter()
        .filter(|(_, parts)| parts.len() == 2)
        .map(|(_, parts)| parts.iter().map(|part| part.value).product::<usize>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part_01(input), 4361);
    }

    #[test]
    fn test_part_02() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part_02(input), 467835);
    }
}
