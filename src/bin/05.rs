advent_of_code::solution!(5);

pub fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges_str, numbers_str) = input.split_once("\n\n").unwrap();
    let ranges = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();
    let numbers = numbers_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    (ranges, numbers)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, numbers) = parse_input(input);
    Some(
        numbers
            .into_iter()
            .filter(|number| {
                ranges
                    .iter()
                    .any(|range| *number >= range.0 && *number <= range.1)
            })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = parse_input(input);
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut count = 0;
    let mut start = 0;
    for range in &ranges {
        if start > range.1 {
            continue;
        }
        if range.0 > start {
            start = range.0;
        }
        count += range.1 - start + 1;
        start = range.1 + 1;
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
