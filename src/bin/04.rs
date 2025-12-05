advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn update_map(map: &mut Vec<Vec<char>>) -> u64 {
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
                    to_remove.push((y, x));
                }
            }
        }
    }
    for (y, x) in &to_remove {
        map[*y][*x] = '.';
    }
    return to_remove.len() as u64;
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = parse_input(input);
    Some(update_map(&mut map))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = parse_input(input);
    let mut found = 0;
    loop {
        let removed = update_map(&mut map);
        if removed == 0 {
            break;
        }
        found += removed;
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
