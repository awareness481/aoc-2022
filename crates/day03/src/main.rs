extern crate encoding;
use encoding::all::ISO_8859_1;
use encoding::{EncoderTrap, Encoding};
use std::ops::ControlFlow;

fn main() {
    const INPUT: &str = include_str!("./input.txt");
    let binding = INPUT.split('\n').collect::<Vec<&str>>();

    println!("part 1: {}", part_1(binding.clone()));
    println!("part 2: {}", part_2(binding.clone()));
}

fn part_1(vec: Vec<&str>) -> i32 {
    let mut sum: i32 = 0;
    vec.iter().for_each(|line| {
        let new_line = line.split_at(line.len() / 2);
        new_line.0.chars().try_for_each(|c| {
            if new_line.1.contains(c) {
                sum += get_character_priority(c);
                ControlFlow::Break(c)
            } else {
                ControlFlow::Continue(())
            }
        });
    });
    sum
}

fn part_2(vec: Vec<&str>) -> i32 {
    let mut sum = 0;
    vec.chunks(3).for_each(|line| {
        line[0].chars().try_for_each(|c| {
            if line[1].contains(c) && line[2].contains(c) {
                sum += get_character_priority(c);
                ControlFlow::Break(c)
            } else {
                ControlFlow::Continue(())
            }
        });
    });
    sum
}

fn get_character_priority(character: char) -> i32 {
    let encoded = ISO_8859_1
        .encode(&character.to_string(), EncoderTrap::Strict)
        .unwrap()[0] as i32;

    if encoded <= 65 || encoded >= 126 {
        return 0;
    }

    if encoded <= 90 {
        return encoded - 38;
    }
    encoded - 96
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 70);
    }
}
