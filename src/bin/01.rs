use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let digits = Regex::new(r"\d").unwrap();
    let mut result = 0;
    for line in input.lines() {
        let mut matches = digits.find_iter(line);
        if let Some(first) = matches.next() {
            let last = matches.last().unwrap_or(first);
            result += format!("{}{}", first.as_str(), last.as_str())
                .parse::<u32>()
                .unwrap();
        }
    }
    Some(result)
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
