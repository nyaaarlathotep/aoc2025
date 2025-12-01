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

pub fn part02(input: &str) -> Result<String, Error>  {
    Ok("solved part 2".to_string())
}

// 单元测试建议直接写在对应天的文件里
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // 读取同目录下的 test 文件
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part01(&input).unwrap(), "3");
    }
}
