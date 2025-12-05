advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut parts = input.split("\n\n");
    let ranges = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let parts = line
                .split("-")
                .map(|part| part.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (parts[0], parts[1])
        })
        .collect::<Vec<(u64, u64)>>();
    let numbers = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut count = 0;
    for number in numbers {
        for range in &ranges {
            if number >= range.0 && number <= range.1 {
                count += 1;
                break;
            }
        }
    }
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut parts = input.split("\n\n");
    let mut ranges = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let parts = line
                .split("-")
                .map(|part| part.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (parts[0], parts[1])
        })
        .collect::<Vec<(u64, u64)>>();

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
