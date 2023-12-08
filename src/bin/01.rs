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
