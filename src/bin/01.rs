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
    for line in input.lines().filter(|line| !line.is_empty()) {
        let matches: Vec<String> = digits
            .find_iter(line)
            .map(|m| m.as_str().to_string())
            .collect();
        let first = matches.first().unwrap().to_owned();
        let last = matches.last().unwrap_or(&first).to_owned();
        let first = map_numbers.get(&first as &str).unwrap() * 10;
        let last = map_numbers.get(&last as &str).unwrap();
        result += first + last;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
