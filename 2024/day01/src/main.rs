use std::usize;

fn main() {
    dbg!(part1(include_str!("./input.txt")));
    dbg!(part2(include_str!("./input.txt")));
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();
    for line in input.lines() {
        let num_pair: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        left_numbers.push(num_pair[0]);
        right_numbers.push(num_pair[1]);
    }
    (left_numbers, right_numbers)
}

fn part1(input: &str) -> usize {
    let (mut first, mut second) = parse_input(input);
    first.sort_by(|a, b| a.cmp(b));
    second.sort_by(|a, b| a.cmp(b));
    let mut sum: usize = 0;
    for i in 0..first.len() {
        sum += (first[i] as isize - second[i] as isize).abs() as usize;
    }
    sum
}

fn part2(input: &str) -> usize {
    let (left_list, right_list) = parse_input(input);
    let mut sum = 0;
    for left_num in left_list {
        let count: usize = right_list.iter().filter(|&&x| x == left_num).count();
        sum += left_num * count;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./example.txt")), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./example.txt")), 31);
    }
}
