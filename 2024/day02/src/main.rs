fn main() {
    dbg!(part1(include_str!("./input.txt")));
    dbg!(part2(include_str!("./input.txt")));
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines() // Split into lines
        .map(|line| {
            line.split_whitespace() // Split each line by whitespace
                .map(|num| num.parse::<i32>().expect("Failed to parse number")) // Parse each number as usize
                .collect() // Collect into a Vec<usize>
        })
        .collect() // Collect all lines into a Vec<Vec<usize>>
}

fn is_safe(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return false;
    }

    let mut direction = 0;
    for window in report.windows(2) {
        let diff = window[1] - window[0];
        if diff == 0 || diff.abs() > 3 {
            return false;
        }
        if direction == 0 {
            direction = diff.signum();
        } else if diff.signum() != direction {
            return false;
        }
    }
    return true;
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_safe(&modified_report) {
            return true;
        }
    }
    return false;
}

fn part1(input: &str) -> usize {
    let reports: Vec<Vec<i32>> = parse(input);
    reports.iter().filter(|report| is_safe(report)).count()
}

fn part2(input: &str) -> usize {
    let reports: Vec<Vec<i32>> = parse(input);
    reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count()
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example1() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn example2() {
        let input = include_str!("./example.txt");
        let result = part2(input);
        assert_eq!(result, 4);
    }
}
