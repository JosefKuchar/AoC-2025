advent_of_code::solution!(6);

fn solve(numbers: &Vec<Vec<u64>>, operations: &Vec<&str>) -> u64 {
    numbers
        .iter()
        .zip(operations.iter())
        .fold(0, |acc, (numbers, operation)| {
            acc + if *operation == "+" {
                numbers.iter().sum::<u64>()
            } else {
                numbers.iter().product::<u64>()
            }
        })
}

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    (0..len)
        .map(|i| v.iter().map(|row| row[i].clone()).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let (operations, numbers) = lines.split_last().unwrap();
    let numbers = numbers
        .iter()
        .map(|line: &Vec<&str>| {
            line.iter()
                .map(|&num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let numbers = transpose(numbers);
    Some(solve(&numbers, operations))
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let (operations, numbers) = lines.split_last().unwrap();
    let chars = numbers
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let operations = operations.split_whitespace().collect::<Vec<&str>>();
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut current: Vec<u64> = Vec::new();
    for i in 0..chars[0].len() {
        let mut number = String::new();
        for j in 0..chars.len() {
            number += &chars[j][i].to_string();
        }
        let number = number.trim();
        if number == "" {
            numbers.push(current);
            current = Vec::new();
            continue;
        }
        current.push(number.parse::<u64>().unwrap());
    }
    numbers.push(current);
    Some(solve(&numbers, &operations))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
