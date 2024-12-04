use aoc_runner_derive::aoc;

use itertools::Itertools;
#[inline]
fn build(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

#[aoc(day1, part1)]
#[must_use]
pub fn part1(input: &str) -> u32 {
    let (left, right) = build(input);
    left.iter()
        .sorted_unstable()
        .zip(right.iter().sorted_unstable())
        .map(|(e1, e2)| e1.abs_diff(*e2))
        .sum()
}

#[aoc(day1, part2)]
#[must_use]
pub fn part2(input: &str) -> u32 {
    let (left, right) = build(input);

    left.iter()
        .map(|e1| {
            let count = right.iter().filter(|e2| e1 == *e2).count() as u32;
            return count * e1;
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../input/test/day1.txt");

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 11);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 31);
    }
}
