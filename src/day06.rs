use core::num;
use std::{os::linux::raw, vec};

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
    let lines = input.lines().collect::<Vec<&str>>();
    let line_len = lines[0].len();
    let mut number_cache = vec![];
    let mut numbers = vec![];
    let mut res = vec![];
    let mut this_op = None;
    let mut space_count = 0;
    let mut raw_numbers = vec![];
    for j in 0..line_len {
        for i in 0..lines.len() {
            let char_byte = lines[i].as_bytes().iter().nth(j).unwrap();
            match char_byte {
                b'0'..=b'9' => {
                    space_count = 0;
                    let c = (char_byte - b'0') as i64;
                    number_cache.push(c);
                }
                b'+' => {
                    space_count = 0;
                    this_op = Some(b'+');
                    if number_cache.is_empty() {
                        continue;
                    }
                    let mut number = 0;
                    for &c in &number_cache {
                        number = number * 10 + c;
                    }
                    numbers.push(number);
                    number_cache.clear();
                }

                b'*' => {
                    space_count = 0;
                    this_op = Some(b'*');
                    if number_cache.is_empty() {
                        continue;
                    }
                    let mut number = 0;
                    for &c in &number_cache {
                        number = number * 10 + c;
                    }
                    numbers.push(number);
                    number_cache.clear();
                }
                b' ' => {
                    space_count += 1;
                    if !number_cache.is_empty() {
                        let mut number = 0;
                        for &c in &number_cache {
                            number = number * 10 + c;
                        }
                        numbers.push(number);
                        number_cache.clear();
                    }
                }
                _ => {
                    return Err("invalid character");
                }
            }
            if space_count >= 6 {
                if !numbers.is_empty() {
                    if let Some(op) = this_op {
                        // 处理之前的操作符
                        match op {
                            b'+' => {
                                let sum: i64 = numbers.iter().sum();
                                res.push(sum);
                            }
                            b'*' => {
                                let prod: i64 = numbers.iter().product();
                                res.push(prod);
                            }
                            _ => {}
                        }
                        raw_numbers.push(numbers);
                        numbers = vec![];
                    }
                    space_count = 0;
                }
            }
        }
    }
    if let Some(op) = this_op {
        // 处理之前的操作符
        match op {
            b'+' => {
                let sum: i64 = numbers.iter().sum();
                res.push(sum);
            }
            b'*' => {
                let prod: i64 = numbers.iter().product();
                res.push(prod);
            }
            _ => {}
        }
        numbers.clear();
    }
    let final_res: i64 = res
        .iter()
        .sum();
    Ok(final_res.to_string())
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
        assert_eq!(part02(&INPUT).unwrap(), "3263827");
    }
}
