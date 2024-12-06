use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    let lines: Vec<String> = input.lines().map(String::from).collect();

    return lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
}

fn is_monotonic(seq: &[i32]) -> bool {
    if seq.len() <= 1 {
        return true;
    }

    let is_increasing = seq[0] < seq[1];
    seq.windows(2).all(|w| {
        (w[0] < w[1]) == is_increasing && (w[1] - w[0]).abs() >= 1 && (w[1] - w[0]).abs() <= 3
    })
}

fn report_is_valid(parts: &Vec<i32>) -> bool {
    is_monotonic(parts)
        || (0..parts.len()).any(|remove_index| {
            let mut modified = parts.clone();
            modified.remove(remove_index);
            is_monotonic(&modified)
        })
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<i32>]) -> u32 {
    let mut safe_count = 0;
    for line in input {
        if is_monotonic(line) {
            safe_count += 1;
        }
    }
    safe_count
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<i32>]) -> u32 {
    let mut safe_count = 0;
    for line in input {
        if report_is_valid(line) {
            safe_count += 1;
        }
    }
    safe_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day2::part1;
    const SAMPLE: &str = include_str!("../input/test/day2.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 4);
    }
}
