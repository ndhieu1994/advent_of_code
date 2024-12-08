use std::collections::{HashMap, HashSet};
use std::ops::{Add, Mul, Sub};
use std::{isize, usize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct VecPoint(isize, isize);

impl Add for VecPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        VecPoint(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for VecPoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        VecPoint(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<isize> for VecPoint {
    type Output = Self;

    fn mul(self, scalar: isize) -> Self::Output {
        VecPoint(self.0 * scalar, self.1 * scalar)
    }
}

fn main() {
    dbg!(part1(include_str!("./input.txt")));
    dbg!(part2(include_str!("./input.txt")));
}

fn part1(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antennas: HashMap<char, Vec<VecPoint>> = HashMap::new();
    let mut antinodes: HashSet<VecPoint> = HashSet::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let c = map[i][j];
            if c != '.' {
                antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push(VecPoint(i as isize, j as isize));
            }
        }
    }

    for (_frequency, antenna_positions) in &antennas {
        for i in 0..antenna_positions.len() {
            for j in 0..antenna_positions.len() {
                if i != j {
                    let antinode =
                        antenna_positions[i] + (antenna_positions[i] - antenna_positions[j]);
                    if 0 <= antinode.0
                        && 0 <= antinode.1
                        && (antinode.0 as usize) < map.len()
                        && (antinode.1 as usize) < map[0].len()
                    {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn part2(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antennas: HashMap<char, Vec<VecPoint>> = HashMap::new();
    let mut antinodes: HashSet<VecPoint> = HashSet::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let c = map[i][j];
            if c != '.' {
                antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push(VecPoint(i as isize, j as isize));
            }
        }
    }

    for (_frequency, antenna_positions) in &antennas {
        for a in 0..antenna_positions.len() {
            antinodes.insert(antenna_positions[a]);
            for b in 0..antenna_positions.len() {
                if a != b {
                    let mut scale = 1;
                    loop {
                        let antinode = antenna_positions[a]
                            + (antenna_positions[a] - antenna_positions[b]) * scale;
                        if 0 <= antinode.0
                            && 0 <= antinode.1
                            && antinode.0 < map.len() as isize
                            && antinode.1 < map[0].len() as isize
                        {
                            antinodes.insert(antinode);
                            scale += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(include_str!("./example.txt")), 14);
        assert_eq!(part1(include_str!("./example_a.txt")), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(include_str!("./example.txt")), 34);
        assert_eq!(part2(include_str!("./example_t.txt")), 9);
    }
}
