advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut found = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '@' {
                let mut count = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        if y as isize + dy < 0
                            || y as isize + dy >= map.len() as isize
                            || x as isize + dx < 0
                            || x as isize + dx >= map[y].len() as isize
                        {
                            continue;
                        }
                        if map[(y as isize + dy) as usize][(x as isize + dx) as usize] == '@' {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    found += 1;
                }
            }
        }
    }
    Some(found)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut found = 0;
    loop {
        let mut to_remove = Vec::new();
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == '@' {
                    let mut count = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            if y as isize + dy < 0
                                || y as isize + dy >= map.len() as isize
                                || x as isize + dx < 0
                                || x as isize + dx >= map[y].len() as isize
                            {
                                continue;
                            }
                            if map[(y as isize + dy) as usize][(x as isize + dx) as usize] == '@' {
                                count += 1;
                            }
                        }
                    }
                    if count < 4 {
                        found += 1;
                        to_remove.push((y, x));
                    }
                }
            }
        }
        if to_remove.is_empty() {
            break;
        }
        for (y, x) in to_remove {
            map[y][x] = '.';
        }
    }
    Some(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
