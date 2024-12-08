fn main() {
    dbg!(part1(include_str!("./input.txt")));
    dbg!(part2(include_str!("./input.txt")));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
struct Guard {
    pos_i: usize,
    pos_j: usize,
    facing: Direction,
}

impl Guard {
    fn move_guard(&mut self, next_i: usize, next_j: usize) {
        self.pos_i = next_i;
        self.pos_j = next_j;
    }

    fn turn_right(&mut self) {
        match self.facing {
            Direction::UP => self.facing = Direction::RIGHT,
            Direction::DOWN => self.facing = Direction::LEFT,
            Direction::LEFT => self.facing = Direction::UP,
            Direction::RIGHT => self.facing = Direction::DOWN,
        }
    }
}

fn part1(input: &str) -> usize {
    let mut maze: Vec<Vec<char>> = parse_input(input);
    let n = maze.len();
    let mut guard: Guard = Guard {
        pos_i: 0,
        pos_j: 0,
        facing: Direction::UP,
    };

    for i in 0..n {
        for j in 0..n {
            if maze[i][j] == '^' {
                guard.pos_i = i;
                guard.pos_j = j;
            }
        }
    }

    while guard.pos_i > 0 && guard.pos_j > 0 && guard.pos_i < n - 1 && guard.pos_j < n - 1 {
        let mut c = '#';
        let (mut next_i, mut next_j) = (0, 0);

        while c == '#' {
            (next_i, next_j) = match guard.facing {
                Direction::UP => (guard.pos_i - 1, guard.pos_j),
                Direction::DOWN => (guard.pos_i + 1, guard.pos_j),
                Direction::LEFT => (guard.pos_i, guard.pos_j - 1),
                Direction::RIGHT => (guard.pos_i, guard.pos_j + 1),
            };
            c = maze[next_i][next_j];
            if c == '#' {
                guard.turn_right();
            }
        }

        maze[guard.pos_i][guard.pos_j] = 'X';
        guard.move_guard(next_i, next_j);
    }

    maze[guard.pos_i][guard.pos_j] = 'X';
    maze.iter()
        .map(|row| row.iter().filter(|c| c == &&'X').count())
        .sum()
}

fn part2(input: &str) -> usize {
    let maze: Vec<Vec<char>> = parse_input(input);
    let n = maze.len();
    let mut start_i = 0;
    let mut start_j = 0;

    let mut working_spots = 0;

    for i in 0..n {
        for j in 0..n {
            if maze[i][j] == '^' {
                start_i = i;
                start_j = j;
            }
        }
    }

    let guard: Guard = Guard {
        pos_i: start_i,
        pos_j: start_j,
        facing: Direction::UP,
    };

    fn check_loop(i: usize, j: usize, guard: &mut Guard, maze: &mut Vec<Vec<char>>) -> bool {
        if maze[i][j] == '#' || maze[i][j] == '^' {
            return false;
        } else {
            maze[i][j] = '#';
        }
        let mut turns = 0;
        while guard.pos_i > 0
            && guard.pos_j > 0
            && guard.pos_i < maze.len() - 1
            && guard.pos_j < maze.len() - 1
        {
            let mut c = '#';
            let (mut next_i, mut next_j) = (guard.pos_i, guard.pos_j);
            while c == '#' {
                (next_i, next_j) = match guard.facing {
                    Direction::UP => (guard.pos_i - 1, guard.pos_j),
                    Direction::DOWN => (guard.pos_i + 1, guard.pos_j),
                    Direction::LEFT => (guard.pos_i, guard.pos_j - 1),
                    Direction::RIGHT => (guard.pos_i, guard.pos_j + 1),
                };
                c = maze[next_i][next_j];
                if c == '#' {
                    guard.turn_right();
                    maze[guard.pos_i][guard.pos_j] = '+';
                }
            }
            guard.move_guard(next_i, next_j);
            if maze[guard.pos_i][guard.pos_j] == '+' {
                turns += 1;
            }
            if turns > 50 {
                return true;
            }
        }
        false
    }

    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if check_loop(i, j, &mut guard.clone(), &mut maze.clone()) {
                working_spots += 1;
            }
        }
    }

    working_spots
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(part1(include_str!("./example.txt")), 41);
    }
    #[test]
    fn example2() {
        assert_eq!(part2(include_str!("./example.txt")), 6);
    }
}
