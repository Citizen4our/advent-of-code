use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Debug)]
struct Row {
    total: u64,
    total_string: String,
    values: Vec<Number>,
}

#[derive(Debug)]
struct Number {
    value: u64,
    count_char: u8,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
    Concatenation,
}

impl Operator {
    pub fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenation => (a.to_string() + &b.to_string()).parse().unwrap(),
        }
    }
}
fn generate_operator_configs(operators: Vec<Operator>, length: usize) -> Vec<Vec<Operator>> {
    std::iter::repeat(operators)
        .take(length - 1)
        .multi_cartesian_product()
        .collect()
}

impl Row {
    pub fn new(row: &str) -> Self {
        let parts: Vec<&str> = row.split(':').collect();
        let total_string = parts[0].trim().to_owned();
        let total = total_string.parse().unwrap();
        let values = parts[1]
            .trim()
            .split(' ')
            .map(|x| Number {
                value: x.parse().unwrap(),
                count_char: x.len() as u8,
            })
            .collect();

        Row {
            total,
            total_string,
            values,
        }
    }
}

fn solve_row(row: &Row, operator_configs: &Vec<Vec<Operator>>) -> Option<u64> {
    let n = row.values.len();

    for ops_config in operator_configs {
        let mut total = row.values[0].value;
        for (i, op) in ops_config.iter().enumerate() {
            total = op.apply(total, row.values[i + 1].value);
        }

        if total == row.total {
            return Some(row.total);
        }
    }

    None
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Row> {
    input.lines().map(Row::new).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Row]) -> u64 {
    let mut ops_config: HashMap<usize, Vec<Vec<Operator>>> = HashMap::new();
    input
        .iter()
        .filter_map(|row| {
            let n = row.values.len();
            let config = ops_config.entry(n).or_insert_with(|| {
                generate_operator_configs(vec![Operator::Add, Operator::Multiply], n)
            });
            solve_row(row, config)
        })
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Row]) -> u64 {
    let mut ops_config: HashMap<usize, Vec<Vec<Operator>>> = HashMap::new();
    input
        .iter()
        .filter_map(|row| {
            let n = row.values.len();
            let config = ops_config.entry(n).or_insert_with(|| {
                generate_operator_configs(
                    vec![Operator::Add, Operator::Multiply, Operator::Concatenation],
                    n,
                )
            });
            solve_row(row, config)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../input/test/day7.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 11387);
    }
}
