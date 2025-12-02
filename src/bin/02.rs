advent_of_code::solution!(2);

use itertools::Itertools;

fn count_digits(num: u64) -> u64 {
    if num == 0 {
        return 1;
    }
    (num as f64).log10() as u64 + 1
}

fn check(id: u64, a: u64) -> bool {
    let exp = 10_u64.pow(a as u32);
    let first = id % exp;
    let mut rest = id / exp;
    while rest > 0 {
        let second = rest % exp;
        if first != second {
            break;
        }
        rest /= exp;
    }
    rest == 0
}

fn is_invalid_part_one(id: u64) -> bool {
    let digits = count_digits(id);
    if digits % 2 != 0 {
        return false;
    }
    check(id, digits / 2)
}

fn is_invalid_part_two(id: u64) -> bool {
    let digits = count_digits(id);
    (1..(digits / 2 + 1)).any(|i| check(id, i))
}

fn solve(input: &str, is_invalid: fn(u64) -> bool) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .flat_map(|(start, end)| (*start..=*end))
            .filter(|num| is_invalid(*num))
            .sum(),
    )
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .filter_map(|range| {
            range
                .split('-')
                .filter_map(|num| num.parse::<u64>().ok())
                .next_tuple::<(u64, u64)>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, is_invalid_part_one)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, is_invalid_part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
