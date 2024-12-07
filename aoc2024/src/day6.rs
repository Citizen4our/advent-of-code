use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::{directions, Matrix};
use std::clone::Clone;
use std::collections::HashSet;
use std::iter::{successors, Iterator};

#[aoc_generator(day6)]
fn parse(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars())).unwrap()
}
const OBSTACLE: char = '#';
#[aoc(day6, part1)]
fn part1(input: &Matrix<char>) -> usize {
    let (start_pos, direction) = find_guard(input);

    let result = do_patrol(input, start_pos, direction);

    if let Some(visited) = result {
        visited
    } else {
        panic!("Loop detected!");
    }
}

fn do_patrol(
    input: &Matrix<char>,
    start_pos: (usize, usize),
    direction: (isize, isize),
) -> Option<usize> {
    let mut turns_positions = HashSet::new();
    let mut is_loop_detected = false;

    let visited = successors(Some((start_pos, direction)), |&(pos, dir)| {
        if turns_positions.contains(&(pos, dir)) {
            is_loop_detected = true;
            None
        } else {
            input.move_in_direction(pos, dir).map(|new_pos| {
                if input.get(new_pos) == Some(&OBSTACLE) {
                    turns_positions.insert((pos, dir));
                    (pos, turn_right(dir))
                } else {
                    (new_pos, dir)
                }
            })
        }
    })
    .take_while(|&(pos, _)| input.get(pos).is_some())
    .map(|(pos, _)| pos)
    .collect::<HashSet<_>>()
    .len();

    if is_loop_detected {
        None
    } else {
        Some(visited)
    }
}

fn find_guard(matrix: &Matrix<char>) -> ((usize, usize), (isize, isize)) {
    for (pos, &value) in matrix.keys().zip(matrix.values()) {
        match value {
            '^' => return (pos, directions::N),
            '>' => return (pos, directions::E),
            'v' => return (pos, directions::S),
            '<' => return (pos, directions::W),
            _ => continue,
        }
    }
    panic!("Couldn't find guard!");
}
fn turn_right(direction: (isize, isize)) -> (isize, isize) {
    match direction {
        directions::N => directions::E,
        directions::E => directions::S,
        directions::S => directions::W,
        directions::W => directions::N,
        _ => panic!("Invalid direction!"),
    }
}

#[aoc(day6, part2)]
fn part2(input: &Matrix<char>) -> usize {
    let (start_pos, direction) = find_guard(input);
    let mut input = input.clone();
    let obstacle = OBSTACLE;

    // its just brute force algorithm....
    input
        .keys()
        .filter_map(|current_position| {
            let previous_char = input.get(current_position)?;
            if previous_char == &obstacle {
                return None;
            }

            let previous_char = *previous_char;
            *input.get_mut(current_position).unwrap() = OBSTACLE;
            let res = do_patrol(&input, start_pos, direction);
            *input.get_mut(current_position).unwrap() = previous_char;

            if res.is_none() {
                Some(())
            } else {
                None
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = include_str!("../input/test/day6.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 6);
    }
}
