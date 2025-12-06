advent_of_code::solution!(6);

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
    let mut sum = 0;
    for (i, operation) in operations.iter().enumerate() {
        if *operation == "+" {
            let mut column = 0;
            for j in 0..numbers.len() {
                column += numbers[j][i];
            }
            sum += column;
        } else if *operation == "*" {
            let mut column = 1;
            for j in 0..numbers.len() {
                column *= numbers[j][i];
            }
            sum += column;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let (operations, numbers) = lines.split_last().unwrap();
    let numbers = numbers
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let operations = operations.split_whitespace().collect::<Vec<&str>>();
    let mut op_index = 0;
    let mut sum = 0;
    let mut current_sum = 0;
    if operations[op_index] == "+" {
        current_sum = 0;
    } else if operations[op_index] == "*" {
        current_sum = 1;
    }
    for i in 0..numbers[0].len() {
        let mut number = String::new();
        for j in 0..numbers.len() {
            number += &numbers[j][i].to_string();
        }
        let number = number.trim();
        if number == "" {
            op_index += 1;
            sum += current_sum;
            if operations[op_index] == "+" {
                current_sum = 0;
            } else if operations[op_index] == "*" {
                current_sum = 1;
            }
            continue;
        }
        let number = number.parse::<u64>().unwrap();
        if operations[op_index] == "+" {
            current_sum += number;
        } else if operations[op_index] == "*" {
            current_sum *= number;
        }
    }
    sum += current_sum;

    Some(sum)
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
