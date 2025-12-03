use std::fmt::Error;

pub fn part01(input: &str) -> Result<String, Error> {
    let res: i64 = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let mut biggest_num: String = chars.by_ref().take(2).collect();
            for c in chars {
                biggest_num = biggest(biggest_num.clone(), c as u8);
            }
            biggest_num.parse::<i64>().unwrap()
        })
        .sum();

    Ok(res.to_string())
}

fn biggest(ori: String, new_char: u8) -> String {
    let first_char = ori.as_bytes()[0];
    let second_char = ori.as_bytes()[1];
    let a = (first_char - b'0') * 10 + new_char - b'0';
    let b = (second_char - b'0') * 10 + new_char - b'0';
    let c = ori.parse::<u8>().unwrap();
    let max = a.max(b).max(c);
    max.to_string()
}

pub fn part02(input: &str) -> Result<String, Error> {
    Ok("solved part 2".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
    #[test]
    fn test_part1() {
        // 读取同目录下的 test 文件

        assert_eq!(part01(&INPUT).unwrap(), "357");
    }
    #[test]
    fn test_part2() {
        // 读取同目录下的 test 文件

        assert_eq!(part02(&INPUT).unwrap(), "6");
    }
}
