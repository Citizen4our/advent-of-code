use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::iter::Iterator;

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let mut pairs_set: HashSet<(u32, u32)> = HashSet::new();
    let mut lists_vec: Vec<Vec<u32>> = Vec::new();

    input.lines().for_each(|line| {
        if line.contains('|') {
            if let Some((a, b)) = parse_pair(&line) {
                pairs_set.insert((a, b));
            }
        } else if line.contains(',') {
            if let Some(list) = parse_list(&line) {
                lists_vec.push(list);
            }
        }
    });

    Input {
        rule_map: pairs_set,
        update_list: lists_vec,
    }
}

fn parse_pair(line: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() == 2 {
        if let (Ok(a), Ok(b)) = (parts[0].trim().parse(), parts[1].trim().parse()) {
            return Some((a, b));
        }
    }
    None
}

fn parse_list(line: &str) -> Option<Vec<u32>> {
    let numbers: Result<Vec<u32>, _> = line.split(',').map(|s| s.trim().parse()).collect();
    numbers.ok()
}

#[derive(Debug)]
struct Input {
    rule_map: HashSet<(u32, u32)>,
    update_list: Vec<Vec<u32>>,
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> u32 {
    let mut valid_update: Vec<&Vec<u32>> = Vec::new();

    for current_vec in &input.update_list {
        let is_valid = get_incorrect_index(input, current_vec);

        if let None = is_valid {
            valid_update.push(current_vec);
        }
    }

    let sum = sum_middle(&mut valid_update);

    sum
}

#[aoc(day5, part2)]
#[must_use]
fn part2(input: &Input) -> u32 {
    let rules = input.rule_map.clone();
    let mut updates = input.update_list.clone();

    updates
        .iter_mut()
        .filter_map(|update| {
            (!update.is_sorted_by(|a, b| !rules.contains(&(*b, *a)))).then_some({
                update.sort_by(|a, b| {
                    if rules.contains(&(*a, *b)) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                });
                u32::from(update[update.len() / 2])
            })
        })
        .sum()
}
fn sum_middle<T>(valid_update: T) -> u32
where
    T: IntoIterator,
    T::Item: AsRef<[u32]>,
{
    valid_update
        .into_iter()
        .map(|v| {
            let v = v.as_ref();
            let index = v[v.len() / 2];
            index
        })
        .sum()
}

fn get_incorrect_index(input: &Input, current_vec: &Vec<u32>) -> Option<usize> {
    let mut incorrect_index = 0;
    let is_valid = current_vec.iter().enumerate().all(|(i, &left)| {
        current_vec[i + 1..].iter().all(|&right| {
            let pair = (left, right);
            if input.rule_map.contains(&pair) {
                true
            } else {
                incorrect_index = i;
                false
            }
        })
    });

    if is_valid {
        None
    } else {
        Some(incorrect_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../input/test/day5.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 123);
    }
}
