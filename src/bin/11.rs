use std::collections::HashMap;

use cached::proc_macro::cached;

advent_of_code::solution!(11);

#[derive(PartialEq, Eq, Hash, Clone)]
struct CacheKey {
    current: String,
    fft_visited: bool,
    dac_visited: bool,
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (start, rest) = line.split_once(':').unwrap();
            let connections = rest.split_whitespace().collect::<Vec<_>>();
            (start, connections)
        })
        .collect::<HashMap<&str, Vec<&str>>>()
}

fn traverse_one(map: &HashMap<&str, Vec<&str>>, current: &str) -> u64 {
    if current == "out" {
        return 1;
    }

    if let Some(connections) = map.get(current) {
        return connections
            .iter()
            .map(|connection| traverse_one(map, connection))
            .sum();
    }

    return 0;
}

#[cached(
    key = "CacheKey",
    convert = r#"{ CacheKey { current: current.to_string(), fft_visited, dac_visited } }"#
)]
fn traverse_two(
    map: &HashMap<&str, Vec<&str>>,
    current: &str,
    fft_visited: bool,
    dac_visited: bool,
) -> u64 {
    if current == "out" {
        return if fft_visited && dac_visited { 1 } else { 0 };
    }

    let fft_visited = fft_visited || current == "fft";
    let dac_visited = dac_visited || current == "dac";
    if let Some(connections) = map.get(current) {
        return connections
            .iter()
            .map(|connection| traverse_two(map, connection, fft_visited, dac_visited))
            .sum();
    }

    return 0;
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);
    Some(traverse_one(&map, "you"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_input(input);
    Some(traverse_two(&map, "svr", false, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
        );
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        );
        assert_eq!(result, Some(2));
    }
}
