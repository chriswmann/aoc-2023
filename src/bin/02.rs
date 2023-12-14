use std::f32::INFINITY;

use regex::Regex;
advent_of_code::solution!(2);

#[derive(Default, Debug)]
pub struct Colours {
    red: u32,
    green: u32,
    blue: u32,
}

pub struct Limits {
    red: u32,
    green: u32,
    blue: u32,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            red: 12,
            green: 13,
            blue: 14,
        }
    }
}

fn get_game_number(line: &str) -> u32 {
    let game_number_regex = Regex::new(r"Game (\d+)").unwrap();
    game_number_regex
        .captures(line)
        .ok_or("No game number")
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let limits = Limits::default();
    let mut total_game_number = 0;
    for line in input.lines() {
        let game_number = get_game_number(line);
        let mut game_possible = true;
        for draw in line
            .split_once(": ")
            .unwrap()
            .1
            .replace(';', ",")
            .split(", ")
        {
            let (num, col) = draw.split_once(' ').unwrap();
            let number = num.parse::<u32>().unwrap();
            // println!("{}", draw);
            let mut colours = Colours::default();
            match col {
                "red" => {
                    colours.red = number;
                }
                "green" => {
                    colours.green = number;
                }
                "blue" => {
                    colours.blue = number;
                }
                _ => panic!("Unknown colour"),
            }
            if (colours.red > limits.red)
                | (colours.green > limits.green)
                | (colours.blue > limits.blue)
            {
                game_possible = false;
            }
        }
        if game_possible {
            total_game_number += game_number;
        }
    }
    Some(total_game_number)
}

struct CubeCounter {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeCounter {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn compare(&mut self, colour: &str, count: u32) {
        match (colour, count) {
            ("red", count) => {
                if count > self.red {
                    self.red = count;
                }
            }
            ("green", count) => {
                if count > self.green {
                    self.green = count;
                }
            }
            ("blue", count) => {
                if count > self.blue {
                    self.blue = count;
                }
            }
            _ => panic!("Unknown colour"),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_game_number = 0;
    for line in input.lines() {
        let mut cube_counter = CubeCounter::new();
        for draw in line
            .split_once(": ")
            .unwrap()
            .1
            .replace(';', ",")
            .split(", ")
        {
            let (count_str, colour) = draw.split_once(' ').unwrap();
            let count = count_str.parse::<u32>().unwrap();
            cube_counter.compare(colour, count);
        }
        let power = cube_counter.red * cube_counter.green * cube_counter.blue;
        total_game_number += power
    }
    Some(total_game_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
