use std::fmt::Error;

pub fn part01(input: &str) -> Result<String, Error> {
    let m = map(input);
    // m to stream
    let res = m
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, &c)| {
                    let mut count = 0;
                    if c == 1 {
                        // check up
                        if i > 0 && m[i - 1][j] == 1 {
                            count += 1;
                        }
                        // check down
                        if i < m.len() - 1 && m[i + 1][j] == 1 {
                            count += 1;
                        }
                        // check left
                        if j > 0 && line[j - 1] == 1 {
                            count += 1;
                        }
                        // check right
                        if j < line.len() - 1 && line[j + 1] == 1 {
                            count += 1;
                        }
                        // check diagonal up-left
                        if i > 0 && j > 0 && m[i - 1][j - 1] == 1 {
                            count += 1;
                        }
                        // check diagonal up-right
                        if i > 0 && j < line.len() - 1 && m[i - 1][j + 1] == 1 {
                            count += 1;
                        }
                        // check diagonal down-left
                        if i < m.len() - 1 && j > 0 && m[i + 1][j - 1] == 1 {
                            count += 1;
                        }
                        // check diagonal down-right
                        if i < m.len() - 1 && j < line.len() - 1 && m[i + 1][j + 1] == 1 {
                            count += 1;
                        }

                        if count < 4 { 1 } else { 0 }
                    } else {
                        0
                    }
                })
                .sum::<i64>()
        })
        .sum::<i64>();

    Ok(res.to_string())
}

fn map(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => 1,
                    '.' => 0,
                    _ => panic!("invalid char"),
                })
                .collect()
        })
        .collect()
}

pub fn part02(input: &str) -> Result<String, Error> {
    let mut m = map(input);
    // m to stream
    let mut total_moved = 0;
    loop {
        let moved: Vec<(usize, usize)> = m
            .iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .map(|(j, &c)| {
                        let mut count = 0;
                        if c == 1 {
                            // check up
                            if i > 0 && m[i - 1][j] == 1 {
                                count += 1;
                            }
                            // check down
                            if i < m.len() - 1 && m[i + 1][j] == 1 {
                                count += 1;
                            }
                            // check left
                            if j > 0 && line[j - 1] == 1 {
                                count += 1;
                            }
                            // check right
                            if j < line.len() - 1 && line[j + 1] == 1 {
                                count += 1;
                            }
                            // check diagonal up-left
                            if i > 0 && j > 0 && m[i - 1][j - 1] == 1 {
                                count += 1;
                            }
                            // check diagonal up-right
                            if i > 0 && j < line.len() - 1 && m[i - 1][j + 1] == 1 {
                                count += 1;
                            }
                            // check diagonal down-left
                            if i < m.len() - 1 && j > 0 && m[i + 1][j - 1] == 1 {
                                count += 1;
                            }
                            // check diagonal down-right
                            if i < m.len() - 1 && j < line.len() - 1 && m[i + 1][j + 1] == 1 {
                                count += 1;
                            }

                            if count < 4 { Some((i, j)) } else { None }
                        } else {
                            None
                        }
                    })
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect();

        if moved.is_empty() {
            break;
        }
        total_moved += moved.len();
        for (i, j) in moved {
            m[i][j] = 0;
        }
    }
    Ok(total_moved.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    #[test]
    fn test_part1() {
        // 读取同目录下的 test 文件

        assert_eq!(part01(&INPUT).unwrap(), "13");
    }
    #[test]
    fn test_part2() {
        // 读取同目录下的 test 文件

        assert_eq!(part02(&INPUT).unwrap(), "43");
    }
}
