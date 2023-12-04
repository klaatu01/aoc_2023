use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = std::time::Instant::now();
    println!("Part 01: {}", part_1(&input));
    println!("Time: {}µs", now.elapsed().as_micros());

    let now = std::time::Instant::now();
    println!("Part 02: {}", part_2(&input));
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part_1(file: &str) -> usize {
    file.lines()
        .map(|line| {
            let chars = line.chars().filter(|x| x.is_digit(10)).collect::<Vec<_>>();
            let first = chars.first().unwrap();
            let last = chars.last().unwrap();
            format!("{}{}", first, last)
        })
        .map(|x| x.parse::<usize>().unwrap())
        .sum()
}

fn part_2(file: &str) -> usize {
    let hash_map = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]
    .into_iter()
    .collect::<HashMap<&str, &str>>();

    file.lines()
        .map(|line| {
            hash_map.iter().fold(line.to_string(), |acc, (k, v)| {
                acc.replace(k, &v.to_string())
            })
        })
        .map(|line| {
            let chars = line.chars().filter(|x| x.is_digit(10)).collect::<Vec<_>>();
            let first = chars.first().unwrap();
            let last = chars.last().unwrap();
            let output = format!("{}{}", first, last);
            output
        })
        .map(|x| x.parse::<usize>().unwrap())
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(super::part_2(input), 281);
    }
}
