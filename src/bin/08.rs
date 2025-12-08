advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq, Clone)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn distance(&self, other: &JunctionBox) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

#[derive(Debug, PartialEq)]
struct JunctionBoxPair {
    first: usize,
    second: usize,
    distance: f64,
}

fn part_one_impl(input: &str, limit: usize) -> Option<u64> {
    let junction_boxes = input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            JunctionBox {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect::<Vec<JunctionBox>>();

    let mut junction_box_pairs = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in i + 1..junction_boxes.len() {
            let distance = junction_boxes[i].distance(&junction_boxes[j]);
            junction_box_pairs.push(JunctionBoxPair {
                first: i,
                second: j,
                distance,
            });
        }
    }

    junction_box_pairs.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    let mut circuits = vec![0; junction_boxes.len()];
    let mut circuit_index = 1;
    for pair in junction_box_pairs.iter().take(limit) {
        if circuits[pair.first] == 0 && circuits[pair.second] == 0 {
            circuits[pair.first] = circuit_index;
            circuits[pair.second] = circuit_index;
            circuit_index += 1;
        } else if circuits[pair.first] == 0 {
            circuits[pair.first] = circuits[pair.second];
        } else if circuits[pair.second] == 0 {
            circuits[pair.second] = circuits[pair.first];
        } else {
            let first_circuit = circuits[pair.first];
            let second_circuit = circuits[pair.second];
            for i in 0..circuits.len() {
                if circuits[i] == second_circuit {
                    circuits[i] = first_circuit;
                }
            }
        }
    }
    let mut sizes = vec![0; circuit_index];
    for circuit in circuits {
        if circuit == 0 {
            continue;
        }
        sizes[circuit] += 1;
    }

    sizes.sort_by(|a, b| b.cmp(a));
    Some(sizes[0] * sizes[1] * sizes[2])
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_impl(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let junction_boxes = input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            JunctionBox {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect::<Vec<JunctionBox>>();

    let mut junction_box_pairs = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in i + 1..junction_boxes.len() {
            let distance = junction_boxes[i].distance(&junction_boxes[j]);
            junction_box_pairs.push(JunctionBoxPair {
                first: i,
                second: j,
                distance,
            });
        }
    }

    junction_box_pairs.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    let mut circuits = vec![0; junction_boxes.len()];
    let mut circuit_index = 1;
    for pair in junction_box_pairs.iter() {
        if circuits[pair.first] == 0 && circuits[pair.second] == 0 {
            circuits[pair.first] = circuit_index;
            circuits[pair.second] = circuit_index;
            circuit_index += 1;
        } else if circuits[pair.first] == 0 {
            circuits[pair.first] = circuits[pair.second];
        } else if circuits[pair.second] == 0 {
            circuits[pair.second] = circuits[pair.first];
        } else {
            let first_circuit = circuits[pair.first];
            let second_circuit = circuits[pair.second];
            for i in 0..circuits.len() {
                if circuits[i] == second_circuit {
                    circuits[i] = first_circuit;
                }
            }
        }

        let first = circuits[0];
        let mut done = true;
        for circuit in &circuits {
            if first != *circuit {
                done = false;
                break;
            }
        }
        if done {
            return Some((junction_boxes[pair.first].x * junction_boxes[pair.second].x) as u64);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_impl(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
