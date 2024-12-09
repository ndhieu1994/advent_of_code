fn main() {
    dbg!(part1(include_str!("./input.txt")));
    dbg!(part2(include_str!("./input.txt")));
}

fn part1(input: &str) -> usize {
    let disk_map: Vec<u32> = input
        .trim()
        .chars()
        .map(|x| x.to_string().parse::<u32>().unwrap())
        .collect();
    let mut disk: Vec<i64> = Vec::new();
    let mut file_flag = true;
    let mut id_counter = 0;
    for digit in disk_map {
        if file_flag {
            for _i in 0..digit {
                disk.push(id_counter);
            }
            id_counter += 1;
            file_flag = false;
        } else {
            for _i in 0..digit {
                disk.push(-1);
            }
            file_flag = true;
        }
    }
    let mut i = 0;
    let mut j = disk.len() - 1;
    loop {
        while disk[i] != -1 {
            i += 1;
        }
        while disk[j] == -1 {
            j -= 1;
        }
        if i >= j {
            break;
        }
        disk.swap(i, j);
    }
    let mut checksum = 0;
    let mut k = 0;
    while disk[k] != -1 {
        checksum += k * disk[k] as usize;
        k += 1;
    }
    checksum
}

fn part2(input: &str) -> usize {
    let disk_map: Vec<u32> = input
        .trim()
        .chars()
        .map(|x| x.to_string().parse::<u32>().unwrap())
        .collect();
    let mut disk: Vec<i64> = Vec::new();
    let mut file_flag = true;
    let mut id_counter = 0;
    for digit in disk_map {
        if file_flag {
            for _i in 0..digit {
                disk.push(id_counter);
            }
            id_counter += 1;
            file_flag = false;
        } else {
            for _i in 0..digit {
                disk.push(-1);
            }
            file_flag = true;
        }
    }
    id_counter -= 1;

    fn find_block(vec: &Vec<i64>, target: i64) -> Option<(usize, usize)> {
        let mut start = None;
        let mut end = None;
        for i in 0..vec.len() {
            if vec[i] == target {
                if start == None {
                    start = Some(i);
                }
                end = Some(i + 1);
            } else if start != None {
                break;
            }
        }
        start.map(|s| (s, end.unwrap()))
    }

    fn find_block_with_len(vec: &Vec<i64>, target: i64, len: usize) -> Option<(usize, usize)> {
        let mut start = None;
        let mut end = None;
        for i in 0..vec.len() {
            if vec[i] == target {
                if start == None {
                    start = Some(i)
                }
                end = Some(i + 1);
            } else if start != None {
                if end.unwrap() - start.unwrap() < len {
                    start = None;
                    end = None;
                } else {
                    break;
                }
            }
        }
        if end.unwrap() - start.unwrap() < len {
            start = None;
            end = None;
        }
        start.map(|s| (s, end.unwrap()))
    }

    fn swap_blocks(vec: &mut Vec<i64>, start1: usize, start2: usize, len: usize) {
        assert!(start1 + len <= start2, "Blocks must not overlap!");
        assert!(
            start1 + len <= vec.len() || start2 + len <= vec.len(),
            "blocks exceed vector length"
        );
        for i in 0..len {
            vec.swap(start1 + i, start2 + i);
        }
    }

    loop {
        let current_id_block = match find_block(&disk, id_counter) {
            Some(t) => t,
            None => {
                panic!("id_counter: id {} not found", id_counter);
            }
        };
        let len = current_id_block.1 - current_id_block.0;

        let free_block = match find_block_with_len(&disk, -1, len) {
            Some(t) => t,
            None => {
                id_counter -= 1;
                continue;
            }
        };

        if free_block.0 < current_id_block.0 {
            swap_blocks(&mut disk, free_block.0, current_id_block.0, len);
        }

        id_counter -= 1;
        if id_counter < 0 {
            break;
        }
    }

    let mut checksum = 0;
    for i in 0..disk.len() {
        if disk[i] != -1 {
            checksum += i * disk[i] as usize;
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(include_str!("./example.txt")), 1928);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(include_str!("./example.txt")), 2858);
    }
}
