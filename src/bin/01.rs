advent_of_code::solution!(1);

struct Rotation {
    value: u32,
    direction: Direction,
}

enum Direction {
    Left,
    Right,
}

struct Dial {
    value: i32,
    pub zeros_move: u64,
    pub zeros_global: u64,
}

impl Dial {
    fn new() -> Self {
        Self {
            value: 50,
            zeros_move: 0,
            zeros_global: 0,
        }
    }
}

impl Dial {
    fn rotate(&mut self, rotation: Rotation) {
        for _ in 0..rotation.value {
            let tick = match rotation.direction {
                Direction::Left => -1,
                Direction::Right => 1,
            };
            self.value += tick;
            self.value = self.value.rem_euclid(100);
            if self.value == 0 {
                self.zeros_move += 1;
            }
        }
        if self.value == 0 {
            self.zeros_global += 1;
        }
    }
}

fn get_rotations(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .filter_map(|line| {
            let mut chars = line.chars();
            let direction = chars.next().map(|c| {
                if c == 'L' {
                    Direction::Left
                } else {
                    Direction::Right
                }
            })?;
            let value = chars.collect::<String>().parse::<u32>().ok()?;
            Some(Rotation { value, direction })
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let rotations = get_rotations(input);
    let mut dial = Dial::new();
    for rotation in rotations {
        dial.rotate(rotation);
    }
    Some(dial.zeros_global)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rotations = get_rotations(input);
    let mut dial = Dial::new();
    for rotation in rotations {
        dial.rotate(rotation);
    }
    Some(dial.zeros_move)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
