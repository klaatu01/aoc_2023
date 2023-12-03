use lazy_static::lazy_static;

lazy_static! {
    static ref ROUND_REGEX: regex::Regex = regex::Regex::new(r"(\d+) (red|blue|green)").unwrap();
    static ref GAME_REGEX: regex::Regex = regex::Regex::new(r"Game (\d+): (.*)").unwrap();
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 01: {}", part_01(&input));
    println!("Part 02: {}", part_02(&input));
}

fn part_01(input: &str) -> u32 {
    let bag = Round {
        red: 12,
        blue: 14,
        green: 13,
    };

    input
        .lines()
        .map(|line| Game::from(line.to_string()))
        .filter(|game| game.is_possible(&bag))
        .map(|game| game.id)
        .sum()
}

fn part_02(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Game::from(line.to_string()))
        .map(|game| game.get_power())
        .sum()
}

struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

impl From<String> for Round {
    fn from(s: String) -> Self {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for cap in ROUND_REGEX.captures_iter(&s) {
            match &cap[2] {
                "red" => red = cap[1].parse().unwrap(),
                "blue" => blue = cap[1].parse().unwrap(),
                "green" => green = cap[1].parse().unwrap(),
                _ => panic!("Invalid color"),
            }
        }

        Round { red, blue, green }
    }
}

impl Round {
    fn is_possible(&self, bag: &Round) -> bool {
        self.red <= bag.red && self.blue <= bag.blue && self.green <= bag.green
    }

    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_possible(&self, bag: &Round) -> bool {
        self.rounds.iter().all(|round| round.is_possible(bag))
    }

    fn get_power(&self) -> u32 {
        self.rounds
            .iter()
            .fold(
                Round {
                    red: 0,
                    blue: 0,
                    green: 0,
                },
                |acc, b| Round {
                    red: acc.red.max(b.red),
                    blue: acc.blue.max(b.blue),
                    green: acc.green.max(b.green),
                },
            )
            .power()
    }
}

impl From<String> for Game {
    fn from(s: String) -> Self {
        let captures = GAME_REGEX.captures_iter(&s).collect::<Vec<_>>();
        let id = captures.first().unwrap()[1].parse().unwrap();
        let rounds = captures.last().unwrap()[2]
            .split("; ")
            .map(|round| Round::from(round.to_string()))
            .collect();

        Game { id, rounds }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_01(input), 8);
    }

    #[test]
    fn test_part_02() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_02(input), 2286);
    }
}
