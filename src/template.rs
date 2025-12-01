pub fn part01(input: &str) -> Result<String, Error> {
    OK(format!("solved part 1 with len {}", input.len()))
}

pub fn part02(input: &str) -> Result<String, Error>  {
    OK("solved part 2".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "";
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