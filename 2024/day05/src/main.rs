use std::{cmp::Ordering, collections::HashMap};

fn main() {
    dbg!(part1(include_str!("./input.txt")));
    dbg!(part2(include_str!("./input.txt")));
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for (key, value) in parts[0]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let rule_nums: Vec<i32> = line
                .split('|')
                .map(|n| n.trim().parse::<i32>().unwrap())
                .collect();
            (rule_nums[0], rule_nums[1])
        })
    {
        rules.entry(key).or_insert_with(Vec::new).push(value);
    }

    let updates: Vec<Vec<i32>> = parts[1]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim()
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn compare_by_rules(a: i32, b: i32, rules: &HashMap<i32, Vec<i32>>) -> Ordering {
    if rules.get(&a).unwrap_or(&vec![]).contains(&b) {
        return Ordering::Less;
    } else if rules.get(&b).unwrap_or(&vec![]).contains(&a) {
        return Ordering::Greater;
    } else {
        return Ordering::Equal;
    }
}

fn part1(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);
    let mut sum = 0;
    'outer: for update in updates {
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                match compare_by_rules(update[i], update[j], &rules) {
                    Ordering::Less => continue,
                    Ordering::Greater => continue 'outer,
                    Ordering::Equal => continue,
                }
            }
        }
        sum += update[update.len() / 2]
    }
    sum
}

fn part2(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);
    let mut sum = 0;
    let mut incorrect = false;
    for mut update in updates {
        'outer: for i in 0..update.len() {
            for j in i + 1..update.len() {
                match compare_by_rules(update[i], update[j], &rules) {
                    Ordering::Less => continue,
                    Ordering::Greater => {
                        incorrect = true;
                        break 'outer;
                    }
                    Ordering::Equal => continue,
                }
            }
        }
        if incorrect {
            update.sort_by(|&a, &b| compare_by_rules(a, b, &rules));
            sum += update[update.len() / 2];
            incorrect = false;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./example.txt")), 143)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./example.txt")), 123)
    }
}
