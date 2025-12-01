pub fn part01(input: &str) -> String {
    // 你的逻辑
    format!("solved part 1 with len {}", input.len())
}

pub fn part02(input: &str) -> String {
    // 你的逻辑
    "solved part 2".to_string()
}

// 单元测试建议直接写在对应天的文件里
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        // 读取同目录下的 test 文件
        let input = fs::read_to_string("./src/day1/test").unwrap();
        assert_eq!(part01(&input), "expected_result");
    }
}