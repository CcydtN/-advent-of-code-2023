advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    fn get_calibration_value(s: &str) -> Option<u32>{
        // print!("{s:?}");
        let mut iter = s.chars().filter(|c| c.is_digit(10));
        let a = iter.next()?.to_digit(10)?;
        let b = iter.last().and_then(|x| x.to_digit(10)).unwrap_or(a);
        Some(10*a+b)
    }
    Some(input.split('\n').filter_map(get_calibration_value).sum::<u32>())
    // None
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
