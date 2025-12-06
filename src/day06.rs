pub fn part01(input: &str) -> Result<String, &str> {
    let (numbers, operators) = parse(input);
    let res = operators
        .iter()
        .enumerate()
        .map(|(idx, &op)| match op {
            "+" => numbers.iter().map(|line| line[idx]).sum::<i64>(),
            "*" => numbers.iter().map(|line| line[idx]).product::<i64>(),
            _ => panic!("unknown operator"),
        })
        .sum::<i64>();
    Ok(res.to_string())
}

fn parse(input: &str) -> (Vec<Vec<i64>>, Vec<&str>) {
    let mut lines = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let operators = lines.pop().unwrap();
    let numbers = lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|&s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    (numbers, operators)
}

pub fn part02(input: &str) -> Result<String, &str> {
    Ok("solved part 2".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    #[test]
    fn test_part1() {
        assert_eq!(part01(&INPUT).unwrap(), "4277556");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part02(&INPUT).unwrap(), "6");
    }
}
