advent_of_code::solution!(3);

pub fn solve(input: &str, length: usize) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect::<Vec<u64>>()
            })
            .map(|line| {
                let mut num = 0;
                let mut start = 0;
                for i in 0..length {
                    let mut max = line[start];
                    let mut max_index = start;
                    for i in start..(line.len() - (length - i) + 1) {
                        if line[i] > max {
                            max = line[i];
                            max_index = i;
                        }
                    }
                    num = 10 * num + max;
                    start = max_index + 1;
                }
                num
            })
            .sum::<u64>(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
