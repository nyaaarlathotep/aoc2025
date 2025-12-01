use std::fmt::Error;

pub fn part01(input: &str) -> Result<String, Error> {
    let mut pos = 50;
    let mut count = 0;
    input.lines().for_each(|line| {
        let (turn, step) = line.split_at(1);
        let step: i64 = step.parse().unwrap();
        match turn {
            "L" => {
                pos = pos - step;
                while pos < 0 {
                    pos = pos + 100;
                }
            }
            "R" => {
                pos = (pos + step) % 100;
            }
            _ => panic!("invalid turn"),
        }
        if pos == 0 {
            count = count + 1;
        }
    });
    Ok(count.to_string())
}

pub fn part02(input: &str) -> Result<String, Error> {
    let mut pos = 50;
    let mut count = 0;
    input.lines().for_each(|line| {
        let (turn, step) = line.split_at(1);
        let step: i64 = step.parse().unwrap();
        let last_pos = pos;
        match turn {
            "L" => {
                pos = pos - step;
                while pos < 0 {
                    pos = pos + 100;
                    count = count + 1;
                }
                if last_pos == 0 {
                    count = count - 1;
                }
            }
            "R" => {
                pos = pos + step;
                while pos >= 100 {
                    if pos != 100 {
                        count = count + 1;
                    }
                    pos = pos - 100;
                }
            }
            _ => panic!("invalid turn"),
        }
        if pos == 0 {
            count = count + 1;
        }
    });
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    #[test]
    fn test_part1() {
        // 读取同目录下的 test 文件

        assert_eq!(part01(&INPUT).unwrap(), "3");
    }
    #[test]
    fn test_part2() {
        // 读取同目录下的 test 文件

        assert_eq!(part02(&INPUT).unwrap(), "6");
    }
}
