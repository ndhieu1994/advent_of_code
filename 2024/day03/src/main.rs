use regex::Regex;

fn main() {
    dbg!(part1(include_str!("./input.txt")));
    dbg!(part2(include_str!("./input.txt")));
}

fn mul(mul_str: &str) -> i32 {
    let result: i32 = mul_str
        .strip_prefix("mul(")
        .and_then(|s| s.strip_suffix(")"))
        .map(|content| {
            content
                .split(',')
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0] * numbers[1])
        .unwrap();
    return result;
}

fn part1(input: &str) -> i32 {
    let mul_regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut sum = 0;
    for mul_match in mul_regex.find_iter(input) {
        sum += mul(&input[mul_match.start()..mul_match.end()]);
    }
    return sum;
}

fn part2(input: &str) -> i32 {
    let mul_dont_do_regex = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();

    let parts: Vec<&str> = mul_dont_do_regex
        .find_iter(input)
        .map(|mat| mat.as_str())
        .collect();

    let mut sum = 0;
    let mut flag = true;
    for part in parts {
        if part == "don't()" {
            flag = false;
        } else if part == "do()" {
            flag = true;
        } else {
            if flag {
                sum += mul(part);
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./example.txt")), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./example.txt")), 48);
    }
}
