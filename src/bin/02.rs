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

pub fn part_one(input: &str) -> Option<u32> {
    let limits = Limits::default();
    let game_number = Regex::new(r"Game (\d+)").unwrap();
    let mut total_game_number = 0;
    for line in input.lines() {
        let mut game_possible = true;
        println!("{}", line);
        let game_number = game_number
            .captures(line)
            .ok_or("No game number")
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        // println!("Game {}", game_number);
        // println!("{:?}", line);
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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
