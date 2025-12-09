advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let coordinates = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();
    let mut max_area = 0;
    for coordinate in &coordinates {
        for other_coordinate in &coordinates {
            if coordinate == other_coordinate {
                continue;
            }
            let area = (coordinate.0 - other_coordinate.0 + 1).abs()
                * (coordinate.1 - other_coordinate.1 + 1).abs();
            if area > max_area {
                max_area = area;
            }
        }
    }
    Some(max_area as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
