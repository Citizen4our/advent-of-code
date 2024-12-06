use aoc_runner_derive::aoc;
use regex::Regex;
use std::sync::LazyLock;

static RE_MUL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

static RE_WITH_DO_DONT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap());
#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    RE_MUL
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x, y])| x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap())
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut enabled = true;

    for m in RE_WITH_DO_DONT.find_iter(input).map(|m| m.as_str()) {
        match m {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => result += part1(m),
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = include_str!("../input/test/day3_1.txt");
    const SAMPLE_2: &str = include_str!("../input/test/day3_2.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&SAMPLE_1), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&SAMPLE_2), 48);
    }
}
