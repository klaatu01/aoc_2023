fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = std::time::Instant::now();
    println!("Part 01: {}", part_01(&input));
    println!("Time: {}µs", now.elapsed().as_micros());

    let now = std::time::Instant::now();
    println!("Part 02: {}", part_02(&input));
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part_01(input: &str) -> usize {
    parse(input)
        .iter()
        .flatten()
        .map(|x| 2_usize.pow(x.len() as u32 - 1))
        .sum()
}

fn part_02(input: &str) -> usize {
    let winning_cards = parse(input);
    let mut wining_instances = vec![1; winning_cards.len() + 1];

    winning_cards
        .iter()
        .enumerate()
        .for_each(|(index, winning)| {
            if let Some(winning) = winning {
                (0..winning.len()).for_each(|i| {
                    wining_instances[index + i + 1] += wining_instances[index];
                });
            }
        });

    wining_instances.iter().skip(1).sum()
}

fn parse(input: &str) -> Vec<Option<Vec<usize>>> {
    input
        .lines()
        .flat_map(|line| line.split(':').last())
        .flat_map(|line| line.split_once("|"))
        .map(|(l, r)| {
            let numbers = l
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let have = r
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let matches = numbers
                .into_iter()
                .filter(|n| have.contains(n))
                .collect::<Vec<usize>>();

            match matches.len() {
                0 => None,
                _ => Some(matches),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_01() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(super::part_01(&input), 13);
    }

    #[test]
    fn part_02() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(super::part_02(&input), 30);
    }
}
