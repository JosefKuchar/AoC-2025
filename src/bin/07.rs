use std::collections::HashSet;

advent_of_code::solution!(7);

fn find_start(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let start = find_start(&map).unwrap();
    let mut sources = vec![start];
    let mut visited = HashSet::new();
    let mut splits = 0;
    while !sources.is_empty() {
        let (i, j) = sources.pop().unwrap();
        visited.insert((i, j));
        if i + 1 >= map.len() {
            continue;
        }
        if visited.contains(&(i + 1, j)) {
            continue;
        }
        match map[i + 1][j] {
            '.' => {
                sources.push((i + 1, j));
            }
            '^' => {
                splits += 1;
                if !sources.contains(&(i + 1, j - 1)) {
                    sources.push((i + 1, j - 1));
                }
                if !sources.contains(&(i + 1, j + 1)) {
                    sources.push((i + 1, j + 1));
                }
            }
            _ => continue,
        }
    }
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let start = find_start(&map).unwrap();
    let mut timelines = vec![0; map[0].len()];
    timelines[start.1] = 1;
    for row in map.iter() {
        let mut new_timelines = vec![0; map[0].len()];
        for (j, count) in timelines.iter().enumerate() {
            match row[j] {
                '^' => {
                    new_timelines[j - 1] += count;
                    new_timelines[j + 1] += count;
                }
                _ => {
                    new_timelines[j] += count;
                }
            }
        }
        timelines = new_timelines;
    }
    Some(timelines.iter().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u64> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
