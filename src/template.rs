pub fn part01(input: &str) -> Result<String, &str> {
    Ok(format!("solved part 1 with len {}", input.len()))
}

pub fn part02(input: &str) -> Result<String, &str> {
    Ok("solved part 2".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "";
    #[test]
    fn test_part1() {

        assert_eq!(part01(&INPUT).unwrap(), "3");
    }
    #[test]
    fn test_part2() {

        assert_eq!(part02(&INPUT).unwrap(), "6");
    }
}
