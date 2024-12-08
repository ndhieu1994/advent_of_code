use std::collections::HashMap;

fn main() {
    dbg!(part1(include_str!("./input.txt")));
    dbg!(part2(include_str!("./input.txt")));
}

fn valid_equation(key: u64, numbers: &[u64]) -> bool {
    fn valid_equation_recursive_helper(key: u64, remaining: &[u64], current_key: u64) -> bool {
        if current_key == key && remaining.len() == 0 {
            return true;
        }
        if remaining.is_empty() {
            return false;
        }
        if current_key > key {
            return false;
        }
        let first = remaining[0];
        let rest = &remaining[1..];
        return valid_equation_recursive_helper(key, rest, current_key + first)
            || valid_equation_recursive_helper(key, rest, current_key * first);
    }
    return valid_equation_recursive_helper(key, &numbers[1..], numbers[0]);
}

fn valid_equation_concat(key: u64, numbers: &[u64]) -> bool {
    fn valid_equation_concat_recursive_helper(
        key: u64,
        remaining: &[u64],
        current_key: u64,
    ) -> bool {
        if current_key == key && remaining.len() == 0 {
            return true;
        }
        if remaining.is_empty() {
            return false;
        }
        if current_key > key {
            return false;
        }
        let first = remaining[0];
        let rest = &remaining[1..];
        return valid_equation_concat_recursive_helper(key, rest, current_key + first)
            || valid_equation_concat_recursive_helper(key, rest, current_key * first)
            || valid_equation_concat_recursive_helper(
                key,
                rest,
                (current_key.to_string() + first.to_string().as_str())
                    .parse::<u64>()
                    .unwrap(),
            );
    }
    return valid_equation_concat_recursive_helper(key, &numbers[1..], numbers[0]);
}

fn part1(input: &str) -> u64 {
    let equations: HashMap<u64, Vec<u64>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let pair_str: Vec<&str> = line.split(':').collect();
            let key = pair_str[0].trim().parse::<u64>().unwrap();
            let value = pair_str[1]
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            (key, value)
        })
        .collect();
    let mut sum = 0;
    for (key, numbers) in equations {
        if valid_equation(key, &numbers) {
            sum += key;
        }
    }
    sum
}

fn part2(input: &str) -> u64 {
    let equations: HashMap<u64, Vec<u64>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let pair_str: Vec<&str> = line.split(':').collect();
            let key = pair_str[0].trim().parse::<u64>().unwrap();
            let value = pair_str[1]
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            (key, value)
        })
        .collect();
    let mut sum = 0;
    for (key, numbers) in equations {
        if valid_equation_concat(key, &numbers) {
            sum += key;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(include_str!("./example.txt")), 3749);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(include_str!("./example.txt")), 11387);
    }
}
