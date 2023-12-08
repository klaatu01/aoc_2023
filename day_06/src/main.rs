use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 01: {}", part_01(input));
    println!("Part 02: {}", part_02(input));
}

fn part_01(input: &str) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let data: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            re.find_iter(line)
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .collect()
        })
        .into_iter()
        .collect();

    let races: Vec<(usize, usize)> = data[0]
        .iter()
        .zip(data[1].iter())
        .map(|(&a, &b)| (a, b))
        .collect();

    races
        .iter()
        .map(|(race_length, race_record)| {
            let breakers: Vec<usize> = (0..*race_length)
                .into_iter()
                .filter_map(|input_time| {
                    let d = input_time * (race_length - input_time);

                    match d > *race_record {
                        true => Some(1),
                        false => None,
                    }
                })
                .collect();

            <usize as TryInto<usize>>::try_into(breakers.len()).unwrap()
        })
        .product()
}

fn part_02(input: &str) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let race: (usize, usize) = input
        .lines()
        .map(|line| re.find_iter(line).map(|x| x.as_str()).collect::<String>())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|x| (x[0], x[1]))
        .collect::<Vec<(usize, usize)>>()[0];

    (0..race.0)
        .filter_map(|input_time| {
            let d = input_time * (race.0 - input_time);

            match d > race.1 {
                true => Some(1),
                false => None,
            }
        })
        .sum()
}
