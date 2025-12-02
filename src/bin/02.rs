advent_of_code::solution!(2);

fn is_invalid(id: u64) -> bool {
    let id = id.to_string();

    if id.len() % 2 != 0 {
        return false;
    }

    id[0..id.len() / 2] == id[id.len() / 2..]
}

fn is_invalid2(id: u64) -> bool {
    let id = id.to_string();

    for i in 1..id.len() {
        if id.len() % i != 0 {
            continue;
        }

        let mut invalid = true;
        let mut offset = 0;
        loop {
            let first = &id[offset..offset + i];
            let second = &id[offset + i..offset + 2 * i];
            if first != second {
                invalid = false;
                break;
            }
            offset += i;
            if offset + i >= id.len() || offset + 2 * i > id.len() {
                break;
            }
        }

        if invalid {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let mut sum = 0;
    for range in ranges {
        let (start, end) = (range[0], range[1]);
        for num in start..=end {
            if is_invalid(num) {
                sum += num;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let mut sum = 0;
    for range in ranges {
        let (start, end) = (range[0], range[1]);
        for num in start..=end {
            if is_invalid2(num) {
                sum += num;
            }
        }
    }
    Some(sum)
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
