use std::collections::HashMap;

use regex::Regex;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    for line in input.lines() {
        let mut digits = Vec::new();
        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                digits.push(digit);
            }
        }
        result += format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<u32>()
            .unwrap();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let digits = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut result = 0;
    let map_numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    for line in input.lines() {
        let mut matches = digits.find_iter(line);
        let first = map_numbers.get(matches.next().unwrap().as_str()).unwrap();
        if let Some(last) = matches.last() {
            result += (first * 10) + map_numbers.get(last.as_str()).unwrap();
        } else {
            result += (first * 10) + first;
        }
    }
    Some(result)
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
