use std::usize;

fn main() {
    dbg!(part1(include_str!("./input.txt")));
    dbg!(part2(include_str!("./input.txt")));
}

static DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),   // Right
    (1, 0),   // Down
    (0, -1),  // Left
    (-1, 0),  // Up
    (1, 1),   // Down-Right (↘)
    (1, -1),  // Down-Left (↙)
    (-1, 1),  // Up-Right (↗)
    (-1, -1), // Up-Left (↖)
];

fn count_xmas(input: &str) -> usize {
    let word_vec: Vec<char> = "XMAS".chars().collect();
    let array: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (m, n) = (array.len(), array[0].len());
    let mut count = 0;

    fn in_bounds(x: i32, y: i32, m: usize, n: usize) -> bool {
        x >= 0 && y >= 0 && (x as usize) < m && (y as usize) < n
    }

    for i in 0..m {
        for j in 0..n {
            for (directionx, directiony) in DIRECTIONS {
                let mut word_found = true;
                for k in 0..word_vec.len() {
                    let x = i as i32 + k as i32 * directionx;
                    let y = j as i32 + k as i32 * directiony;
                    if !in_bounds(x, y, m, n) {
                        word_found = false;
                        break;
                    } else if in_bounds(x, y, m, n) {
                        if array[x as usize][y as usize] != word_vec[k] {
                            word_found = false;
                            break;
                        }
                    }
                }

                if word_found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_x_mas(input: &str) -> usize {
    let array: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (m, n) = (array.len(), array[0].len());
    let mut count = 0;

    for i in 1..(m - 1) {
        for j in 1..(n - 1) {
            if array[i][j] == 'A' {
                if (array[i - 1][j - 1] == 'M' && array[i + 1][j + 1] == 'S'
                    || array[i - 1][j - 1] == 'S' && array[i + 1][j + 1] == 'M')
                    && (array[i + 1][j - 1] == 'M' && array[i - 1][j + 1] == 'S'
                        || array[i + 1][j - 1] == 'S' && array[i - 1][j + 1] == 'M')
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part1(input: &str) -> usize {
    count_xmas(input)
}

fn part2(input: &str) -> usize {
    count_x_mas(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./example.txt")), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./example.txt")), 9);
    }
}
