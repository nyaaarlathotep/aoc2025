use std::collections::{HashMap, HashSet};

pub fn part01(input: &str) -> Result<String, &str> {
    let map = parse_input(input);
    let start = map[0]
        .iter()
        .enumerate()
        .find(|&(_, &c)| c == 'S')
        .map(|(i, _)| i)
        .unwrap();
    let mut light_poses = HashSet::new();
    light_poses.insert(start);
    let line_len = map[0].len();
    let mut split_count = 0;
    for row in &map[1..] {
        let mut new_light_poses = HashSet::with_capacity(line_len);
        for &pos in &light_poses {
            if row[pos] == '^' {
                split_count += 1;
                if pos > 0 {
                    new_light_poses.insert(pos - 1);
                }
                if pos < line_len - 1 {
                    new_light_poses.insert(pos + 1);
                }
            } else {
                new_light_poses.insert(pos);
            }
        }
        light_poses = new_light_poses;
    }

    Ok(split_count.to_string())
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part02(input: &str) -> Result<String, &str> {
    let map = parse_input(input);
    let start = map[0]
        .iter()
        .enumerate()
        .find(|&(_, &c)| c == 'S')
        .map(|(i, _)| i)
        .unwrap();
    let mut light_poses = HashMap::with_capacity(1);
    light_poses.insert(start, 1u64);
    let line_len = map[0].len();
    let mut split_count = 1;
    for row in &map[1..] {
        let mut new_light_poses = HashMap::with_capacity(line_len);
        for (&pos, &count) in &light_poses {
            if row[pos] == '^' {
                split_count += count;
                if pos > 0 {
                    new_light_poses.insert(
                        pos - 1,
                        new_light_poses.get(&(pos - 1)).unwrap_or(&0) + count,
                    );
                }
                if pos < line_len - 1 {
                    new_light_poses.insert(
                        pos + 1,
                        new_light_poses.get(&(pos + 1)).unwrap_or(&0) + count,
                    );
                }
            } else {
                new_light_poses.insert(pos, new_light_poses.get(&pos).unwrap_or(&0) + count);
            }
        }
        light_poses = new_light_poses;
    }

    Ok(split_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    #[test]
    fn test_part1() {
        assert_eq!(part01(&INPUT).unwrap(), "21");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part02(&INPUT).unwrap(), "40");
    }
}
