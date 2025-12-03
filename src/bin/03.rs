advent_of_code::solution!(3);

pub fn find_max(numbers: &Vec<u64>, start: usize, end: usize) -> (u64, usize) {
    let mut max = numbers[start];
    let mut max_index = start;
    for i in start..end {
        if numbers[i] > max {
            max = numbers[i];
            max_index = i;
        }
    }
    (max, max_index)
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let mut sum: u64 = 0;
    for line in lines {
        let mut max: u64 = 0;
        let mut max_index = 0;
        for i in 0..(line.len() - 1) {
            if line[i] > max {
                max = line[i];
                max_index = i;
            }
        }
        let mut second_max: u64 = 0;
        for i in (max_index + 1)..line.len() {
            if line[i] > second_max {
                second_max = line[i];
            }
        }

        sum += 10 * max + second_max;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let mut sum: u64 = 0;
    for line in lines {
        let mut num = 0;
        let mut start = 0;
        for i in 0..12 {
            let (max, max_index) = find_max(&line, start, line.len() - (12 - i));
            num = 10 * num + max;
            start = max_index + 1;
        }
        sum += num;
    }
    Some(sum)
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
